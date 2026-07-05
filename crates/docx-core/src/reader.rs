use std::io::{Cursor, Read};
use zip::ZipArchive;
use quick_xml::Reader;
use quick_xml::events::Event;

/// Содержимое DOCX-файла после парсинга.
#[derive(Debug, Clone)]
pub struct DocxContent {
    /// Полный текст документа (все параграфы слиты с пробелами).
    pub full_text: String,
    /// Список тегов вида `{{ tag_name }}`, найденных в тексте.
    pub tags: Vec<String>,
    /// Количество параграфов (тегов <w:p>).
    pub paragraph_count: usize,
}

impl DocxContent {
    /// Принимает байты DOCX-файла, возвращает структуру с данными.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DocxError> {
        // 1. Открываем ZIP-архив
        let cursor = Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor)
            .map_err(|e| DocxError::ZipError(e.to_string()))?;

        // 2. Ищем файл word/document.xml
        let doc_xml = {
            let mut file = archive.by_name("word/document.xml")
                .map_err(|e| DocxError::NotDocx(e.to_string()))?;
            let mut xml = String::new();
            file.read_to_string(&mut xml)
                .map_err(|e| DocxError::IoError(e.to_string()))?;
            xml
        };

        // 3. Парсим XML и извлекаем текст
        let mut reader = Reader::from_str(&doc_xml);
        reader.config_mut().trim_text(true);

        let mut full_text = String::new();
        let mut paragraph_count = 0;
        let mut in_text = false;
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name_bytes = e.name();
                    let name = String::from_utf8_lossy(name_bytes.as_ref());
                    if name == "p" || name == "w:p" {
                        paragraph_count += 1;
                        if !full_text.is_empty() {
                            full_text.push(' ');
                        }
                    }
                    if name == "t" || name == "w:t" {
                        in_text = true;
                    }
                }
                Ok(Event::Text(ref e)) => {
                    if in_text {
                        let text = e.unescape().unwrap_or_default();
                        full_text.push_str(&text);
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name_bytes = e.name();
                    let name = String::from_utf8_lossy(name_bytes.as_ref());
                    if name == "t" || name == "w:t" {
                        in_text = false;
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(DocxError::XmlError(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        // 4. Находим теги {{ ... }}
        let tags = find_tags(&full_text);

        Ok(DocxContent {
            full_text,
            tags,
            paragraph_count,
        })
    }
}

/// Ищет в тексте все теги вида `{{ tag_name }}`.
fn find_tags(text: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let mut start = 0;

    while let Some(tag_start) = text[start..].find("{{") {
        let abs_start = start + tag_start;
        if let Some(tag_end) = text[abs_start..].find("}}") {
            let abs_end = abs_start + tag_end + 2;
            let tag = text[abs_start + 2..abs_end - 2].trim().to_string();
            if !tag.is_empty() {
                tags.push(tag);
            }
            start = abs_end;
        } else {
            break;
        }
    }

    tags
}

/// Ошибки, возникающие при работе с DOCX.
#[derive(Debug, Clone)]
pub enum DocxError {
    ZipError(String),
    NotDocx(String),
    IoError(String),
    XmlError(String),
}

impl std::fmt::Display for DocxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocxError::ZipError(msg) => write!(f, "Ошибка чтения ZIP: {}", msg),
            DocxError::NotDocx(msg) => write!(f, "Файл не является DOCX: {}", msg),
            DocxError::IoError(msg) => write!(f, "Ошибка ввода-вывода: {}", msg),
            DocxError::XmlError(msg) => write!(f, "Ошибка парсинга XML: {}", msg),
        }
    }
}

impl std::error::Error for DocxError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};
    use zip::write::ZipWriter;

    /// Создаёт минимальный DOCX в памяти с заданным XML-контентом.
    fn create_docx(document_xml: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut zip = ZipWriter::new(Cursor::new(&mut buf));

        // word/document.xml
        zip.start_file("word/document.xml", FileOptions::<()>::default())
            .unwrap();
        zip.write_all(document_xml.as_bytes()).unwrap();

        // [Content_Types].xml — обязателен для валидного DOCX
        zip.start_file::<&str, ()>("[Content_Types].xml", ())
            .unwrap();
        zip.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>...")
            .unwrap();

        zip.finish().unwrap();
        buf
    }

    #[test]
    fn test_parse_simple_docx() {
        let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r>
        <w:t>Hello {{ name }}!</w:t>
      </w:r>
    </w:p>
    <w:p>
      <w:r>
        <w:t>Your order {{ order_id }} is ready.</w:t>
      </w:r>
    </w:p>
  </w:body>
</w:document>
"#;
        let docx_bytes = create_docx(xml);
        let result = DocxContent::from_bytes(&docx_bytes);

        assert!(result.is_ok());
        let content = result.unwrap();
        assert_eq!(content.paragraph_count, 2);
        assert!(content.full_text.contains("Hello"));
        assert!(content.full_text.contains("ready."));
        assert_eq!(content.tags.len(), 2);
        assert!(content.tags.contains(&"name".to_string()));
        assert!(content.tags.contains(&"order_id".to_string()));
    }

    #[test]
    fn test_no_tags() {
        let xml = r#"
<?xml version="1.0" encoding="UTF-8"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r>
        <w:t>Plain text without tags.</w:t>
      </w:r>
    </w:p>
  </w:body>
</w:document>
"#;
        let docx_bytes = create_docx(xml);
        let result = DocxContent::from_bytes(&docx_bytes);
        assert!(result.is_ok());
        assert!(result.unwrap().tags.is_empty());
    }
}