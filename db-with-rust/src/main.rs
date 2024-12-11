use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use std::env;
use std::io::{self, Write};

struct Club {
    clubid: i32,
    clubname: String,
    location: Option<String>,
    homepage: Option<String>,
    leaderid: Option<String>,
    professorid: Option<String>,
}

fn main() {
    dotenv().ok(); // .env 파일 불러오기

    match login_db() {
        Ok(pool) => {
            match pool.get_conn() {
                Ok(mut conn) => {
                    println!("\n동아리 관리 시스템에 성공적으로 연결되었습니다.");
                    'menu: loop {
                        print_menu();
                        
                        let mut user_input = String::new();
                        io::stdin()
                            .read_line(&mut user_input)
                            .expect("failed to read line");
                        let user_input = user_input.trim();
                        
                        match user_input {
                            "99" => {
                                println!("프로그램을 종료합니다...");
                                break;
                            },
                            "1" => {
                                match retriever_club_table(&mut conn) {
                                    Ok(_) => {},
                                    Err(_) => println!("동아리 전체 목록을 조회하는 과정에서 에러가 발생했습니다.")
                                }
                            },
                            _ => println!("잘못 입력했습니다. 메뉴의 번호를 확인해주세요."),
                        }
                    }
                },
                Err(_) => {
                    println!("동아리 관리 시스템에 접근하는 과정에서 예기치 못한 에러가 발생했습니다.");
                    println!("프로그램을 종료합니다...");
                    std::process::exit(1);   
                }
            }
        }
        Err(_) => { }
    }
}

fn print_menu() {
    println!("                                                            ");
    println!("                                                            ");
    println!("------------------------------------------------------------");
    println!("                 소프트웨어학부 동아리 관리 시스템                  ");
    println!("------------------------------------------------------------");
    println!("  1. 동아리 전체 목록 조회            2.                        ");
    println!("  3.                             4.                        ");
    println!("  5.                             6.                        ");
    println!("  7.                             8.                        ");
    println!("  9.                            10.                        ");
    println!(" 11.                            12.                        ");
    println!("                                99. quit                   ");
    println!("------------------------------------------------------------");
    print!("이동을 원하는 메뉴를 선택해주세요: ");
    let _ = io::stdout().flush();
}

fn login_db() -> std::result::Result<mysql::Pool, Box<dyn std::error::Error>> {
    loop {
        println!("                                                            ");
        println!("                                                            ");
        println!("------------------------------------------------------------");
        println!("                 소프트웨어학부 동아리 관리 시스템                  ");
        println!("------------------------------------------------------------");
        println!(" 시스템에 접근하려면 로그인이 필요합니다                              ");
        println!("                                            99: 시스템 종료   ");
        println!("------------------------------------------------------------");
        print!(" ID를 입력해주세요: ");
        let _ = io::stdout().flush();
        let mut id = String::new();
        io::stdin()
            .read_line(&mut id)
            .expect("failed to read line");
        let id = id.trim();
        
        if id == "99" {
            println!("프로그램을 종료합니다...");
            std::process::exit(0);
        }

        print!(" PW를 입력해주세요: ");
        let _ = io::stdout().flush();
        let mut pw = String::new();
        io::stdin()
            .read_line(&mut pw)
            .expect("failed to read line");
        let pw = pw.trim();

        if pw == "99" {
            println!("프로그램을 종료합니다...");
            std::process::exit(0);
        }

        let database_ip = env::var("DATABASE_IP").expect("not set the DATABASE_IP");
        let database_name = env::var("DATABASE_NAME").expect("not set the DATABASE_NAME");

        let db_url = format!("mysql://{}:{}@{}/{}", id, pw, database_ip, database_name);

        match mysql::Pool::new(db_url) {
            Ok(pool) => {
                return Ok(pool);
            }
            Err(err) => {
                println!("\n ID 혹은 PW가 일치하지 않습니다. ID와 PW를 확인해주세요.");
            }
        }
    }
}

fn retriever_club_table(conn:&mut  PooledConn) -> std::result::Result<(), Box<dyn std::error::Error>>{
    println!("\n동아리 전체 목록을 출력합니다.");
    let result = conn.query_map(
        "SELECT * FROM Club",
        |(clubid, clubname, location, homepage, leaderid, professorid)| Club {
            clubid,
            clubname,
            location,
            homepage,
            leaderid,
            professorid,
        },
    )?;
    for r in result {
        println!(
            "{} {} {} {} {} {}",
            r.clubid,
            r.clubname,
            r.location.unwrap_or("NULL".to_string()),
            r.homepage.unwrap_or("NULL".to_string()),
            r.leaderid.unwrap_or("NULL".to_string()),
            r.professorid.unwrap_or("NULL".to_string()),
        );
    }

    Ok(())
}