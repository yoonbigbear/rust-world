use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct SessionManager {
    pub session_container :  HashMap<i32, Arc<Mutex<TcpStream>>>,
    pub session_id : i32,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            session_container: HashMap::new(),
            session_id: 0,
        }
    }

    pub async fn add_session(manager: Arc<Mutex<Self>>, stream: TcpStream) {
        let mut manager_lock = manager.lock().await;
        manager_lock.session_id += 1;

        let session_id = manager_lock.session_id;
        // TcpStream을 Arc<Mutex<TcpStream>>으로 감싸서 HashMap에 저장
        let stream = Arc::new(Mutex::new(stream));
        manager_lock.session_container.insert(session_id, Arc::clone(&stream));

        let manager_clone = Arc::clone(&manager);
        tokio::spawn(async move {
            // read_message 함수를 호출하여 클라이언트로부터 메시지를 받아옴
            Self::read_message(manager_clone, Arc::clone(&stream)).await;
        });
    }

    async fn read_message(manager: Arc<Mutex<Self>>, stream: Arc<Mutex<TcpStream>>) {
        let mut buffer = [0; 1024];
        loop {

            let n = {
            // TcpStream을 lock하여 데이터를 읽어옴. 그리고 잠금의 범위를 최소화 하기 위해 블록을 사용
                let mut locked_stream = stream.lock().await;
                locked_stream.read(&mut buffer).await.unwrap()
            };

            if n == 0 {
                break;
            }

            println!("Received: {:?}", &buffer[..n]);

            {
                // echo back. 다시 잠금의 범위를 최소화 하기 위해 블록을 사용
                let mut locked_stream = stream.lock().await;
                locked_stream.write_all(&buffer[..n]).await.unwrap();
            }

            {
                // 브로드 캐스트를 하기 위한 잠금 처리
                let manager = manager.lock().await;
                for (id, s) in &manager.session_container {
                    if id != &manager.session_id {
                        let mut s = s.lock().await;
                        s.write_all(&buffer[..n]).await.unwrap();
                    }
                }
            }
        }
    }
}