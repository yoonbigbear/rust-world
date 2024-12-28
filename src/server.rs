use crate::ThreadPool;

use std::{
    net::TcpListener,
    net::TcpStream,
    io::{BufRead, BufReader, Write},  // BufRead, Write 추가
    fs,
};

pub struct Server {
    pub ip_addr: String
}

impl Server
{
    /// &self로 받아서, self.ip_addr를 참조할 수 있게 함
    pub fn listen(&mut self) -> TcpListener {
        let listener = TcpListener::bind(
            &self.ip_addr).unwrap();
        let pool = ThreadPool::new(4);

        for stream in listener.incoming()
        {
            let stream = stream.unwrap();
            
            // handle_connection은 동일 impl의 private 메서드
            // 클로저 안에서 직접 호출 가능하지만, self에 접근 X
            // => Self::handle_connection(..) 형태로 부릅니다.
            pool.execute(|| {
                Self::handle_connection(stream);
            });
        }
        listener
    }

    fn handle_connection(mut stream: TcpStream)
    {
        let buf_reader = BufReader::new(&mut stream);
        // lines() 사용 → BufRead 트레이트 필요
        let request_line = buf_reader.lines().next().unwrap().unwrap();
    
        let (status_line, filename) = 
        if request_line == "GET / HTTP/1.1" 
        {
            ("HTTP/1.1 200 OK", "hello.html")
        } 
        else 
        {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    
        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
    
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}