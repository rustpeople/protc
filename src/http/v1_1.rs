use std::collections::HashMap;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub fields: HashMap<String, String>,
    pub body: String
}

impl Request {
    pub fn parse(request_str: &str) -> Result<Request, String> {
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

        Ok(Request {
            method,
            path,
            fields,
            body,
        })
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


// https://httpwg.org/specs/rfc9112.html

/* Here's an example HTTP response:
    HTTP/1.1 200 OK
    Date: Wed, 07 Jun 2023 14:48:22 GMT
    Expires: -1
    Cache-Control: private, max-age=0
    Content-Type: text/html; charset=ISO-8859-1
    Content-Security-Policy-Report-Only: object-src 'none';base-uri 'self';script-src 'nonce-qBsIjUDPFeGKAEJ2e0m0fQ' 'strict-dynamic' 'report-sample' 'unsafe-eval' 'unsafe-inline' https: http:;report-uri https://csp.withgoogle.com/csp/gws/other-hp
    P3P: CP="This is not a P3P policy! See g.co/p3phelp for more info."
    Server: gws
    X-XSS-Protection: 0
    X-Frame-Options: SAMEORIGIN
    Set-Cookie: SOCS=CAAaBgiAkf-jBg; expires=Sat, 06-Jul-2024 14:48:22 GMT; path=/; domain=.google.com; Secure; SameSite=laxSet-Cookie: AEC=AUEFqZf6Lck5tBWPZM5vziyEXBoD1mMr2o9dsy965nGeG1ZZnobthion7pw; expires=Mon, 04-Dec-2023 14:48:22 GMT; path=/; domain=.google.com; Secure; HttpOnly; SameSite=lax
    Set-Cookie: __Secure-ENID=12.SE=lI_v9wyNJ_sFQBuAVi8Q73Ta08LVuJRzNTlDrrJcwuyZyGaY02D_GM0O9mzxfYrWdR6uEZwj6V5Zk1dKlYRd81XKKcNgJ9gdXHM6ElrlnattmX3T1N-dVvKLOTQcdhWQ5m6CJCuJNiNk-qbuLD9xPjhygw98vDYW51Ey96qg-UE; expires=Sun, 07-Jul-2024 07:06:40 GMT; path=/; domain=.google.com; Secure; HttpOnly; SameSite=lax
    Set-Cookie: CONSENT=PENDING+473; expires=Fri, 06-Jun-2025 14:48:22 GMT; path=/; domain=.google.com; Secure
    Alt-Svc: h3=":443"; ma=2592000,h3-29=":443"; ma=2592000
    Accept-Ranges: none
    Vary: Accept-Encoding
    Transfer-Encoding: chunked

    <!doctype html><html itemscope="" itemtype="http://schema.org/WebPage" lang="cs"><head><meta content="text/html; charset=UTF-8" http-equiv="Content-Type"><meta content="/images/branding/googleg/1x/googleg_standard_color_128dp.png" itemprop="image"><title>Google</title><script nonce="qBsIjUDPFeGKAEJ2e0m0fQ">(function(){var _g={kEI:'tpiAZK7kMYqCxc8P4L-aoAg',kEXPI:'0,1359409,6058,207,4804,2316,383,246,5,1129120,1703,1196054,637,380097,16114,28684,22430,1362,12313,17586,4998,50699,4820,2872,2891,8348,3406,606,30668,30022,16335,20583,4,1528,2304,42127,13658,4437,22583,6654,7596,1,11943,30214,2,3356,36402,5679,1021,31122,4568,6258,23418,1252,5835,12141,2827,4332,7484,25076,2006,5895,2260,7381,15970,872,6578,13057,6,1922,9779,42459,2007,1135,17056,928,19305,20206,3371,5006,8048,10912,3883,1520,1506,1524,6111,9705,1804,10472,2885,161,2335,6984,1711,8549,1896,9062,44,2321,662,1632,2483,3318,2,2148,4467,1442,1129,994,4907,2117,257,355,1109,1079,895,2129,5780,2,1050,282
 */