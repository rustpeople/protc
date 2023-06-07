use std::collections::HashMap;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub fields: HashMap<String, String>,
    pub body: String
}

impl Request {
    pub fn parse(request_str: &str) -> Request {
        // Split the request into lines
        let lines: Vec<&str> = request_str.lines().collect();

        // Extract the method and path from the first line
        let first_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if first_line_parts.len() != 3 {
            return Err("Invalid request".to_string());
        }
        let method = first_line_parts[0].to_string();
        let path = first_line_parts[1].to_string();

        // Parse the headers and body
        let mut fields = HashMap::new();
        let mut body = String::new();
        let mut parsing_body = false;

        for line in lines.iter().skip(1) {
            if parsing_body {
                // Add the line to the body
                body.push_str(line);
                body.push_str("\n");
            } else if line.is_empty() {
                // Empty line indicates end of headers
                parsing_body = true;
            } else {
                // Parse headers
                let header_parts: Vec<&str> = line.splitn(2, ": ").collect();
                if header_parts.len() != 2 {
                    return Err("Invalid header format".to_string());
                }
                let key = header_parts[0].to_string();
                let value = header_parts[1].to_string();
                fields.insert(key, value);
            }
        }

        Request {
            method,
            path,
            fields,
            body,
        }
    }
    pub fn to_string(self) -> String {
        let mut result = String::from(format!("{} {} HTTP/1.1\r\n", self.method, self.path));
        for i in self.fields.keys() {
            result += format!("{}: {}\r\n", i, self.fields.get(i).unwrap()).as_str();
        }
        result += format!("\r\n{}", self.body).as_str();
        result
    }
}
// I'm back
// I'll have to take a break+
// I'll make the parser but publish the repo first
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Response {
    pub status: u16,
    pub fields: HashMap<String, String>,
    pub body: String
}

impl Response {
    pub fn parse(data: String) -> Self {
        // Split the request into lines
        let lines: Vec<&str> = request_str.lines().collect();

        // Extract the method and path from the first line
        let first_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if first_line_parts.len() != 2 && first_line_parts.len() != 3  {
            return Err("Invalid request".to_string());
        }

        let status = first_line_parts[1].to_string() as u16;

        let mut fields = HashMap::new();
        let mut body = String::new();
        let mut parsing_body = false;

        for line in lines.iter().skip(1) {
            if parsing_body {
                // Add the line to the body
                body.push_str(line);
                body.push_str("\n");
            } else if line.is_empty() {
                // Empty line indicates end of headers
                parsing_body = true;
            } else {
                // Parse headers
                let header_parts: Vec<&str> = line.splitn(2, ": ").collect();
                if header_parts.len() != 2 {
                    return Err("Invalid header format".to_string());
                }
                let key = header_parts[0].to_string();
                let value = header_parts[1].to_string();
                fields.insert(key, value);
            }
        }

        Self { status: status, fields: fields, body: body }
    }
    pub fn to_string(self) -> String {
        let mut result = String::from(format!("HTTP/1.1 {} \r\n"));
        for i in self.fields.keys() {
            result += format!("{}: {}\r\n", i, self.fields.get(i).unwrap()).as_str()
        }
        result += format!("\r\n{}", self.body).as_str();
        result
    }
}
// https://httpwg.org/specs/rfc9112.html
