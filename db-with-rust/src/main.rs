use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use chrono::NaiveDate;
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

struct Professor {
    profid: String,
    profname: String,
    phone: String,
}

struct Member {
    studentid: String,
    name: String,
    dept: Option<String>,
    gender: Option<String>,
    birth: String,
    phone: Option<String>,
    joindate: String,
    isleader: bool,
    clubid: i32,
}

enum TableList {
    Professor,
    Club,
    Member,
    // Project,
    // ProjectParticipation,
    // Post,
    // Comment,
    // Budget,
}

impl TableList {
    fn table_name(&self) -> &'static str {
        match self {
            TableList::Professor => "Professor",
            TableList::Club => "Club",
            TableList::Member => "Member",
        }
    }
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
                        
                        let mut user_input = get_input();
                        
                        match user_input.as_str() {
                            "99" => {
                                println!("프로그램을 종료합니다...");
                                break;
                            },
                            "1" => {
                                club_management(&mut conn);
                            },
                            "2" => {
                                prof_manament(&mut conn);
                            }
                            "3" => {
                                mem_management(&mut conn);
                            }
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
    println!("  1. 동아리 관리            2. 지도교수 관리                      ");
    println!("  3. 동아리원 관리          4. 프로젝트 관리                      ");
    println!("  5. 게시글 관리            6. 예산 관리                        ");
    println!("  7.                             8.                        ");
    println!("  9.                            10.                        ");
    println!("                                99. 시스템 종료               ");
    println!("------------------------------------------------------------");
    print!("이동을 원하는 메뉴를 선택해주세요: ");
    let _ = io::stdout().flush();
}

fn print_mem_menu() {
    println!("                                                            ");
    println!("                                                            ");
    println!("------------------------------------------------------------");
    println!("        소프트웨어학부 동아리 관리 시스템 - 동아리원 관리               ");
    println!("------------------------------------------------------------");
    println!("  1. 동아리원 전체 목록 조회            2. 동아리원 신규 등록         ");
    println!("  3. 동아리원 검색                    4. 동아리원 정보 변경         ");
    println!("  5. 동아리원 정보 삭제                                         ");
    println!("                                99. 이전 메뉴로 이동            ");
    println!("------------------------------------------------------------");
    print!("이동을 원하는 메뉴를 선택해주세요: ");
    let _ = io::stdout().flush();
}

fn print_prof_menu() {
    println!("                                                            ");
    println!("                                                            ");
    println!("------------------------------------------------------------");
    println!("          소프트웨어학부 동아리 관리 시스템 - 교수 관리               ");
    println!("------------------------------------------------------------");
    println!("  1. 교수 전체 목록 조회               2. 교수 신규 등록           ");
    println!("  3. 교수 검색                       4. 교수 정보 변경           ");
    println!("  5. 교수 정보 삭제                                            ");
    println!("                                99. 이전 메뉴로 이동            ");
    println!("------------------------------------------------------------");
    print!("이동을 원하는 메뉴를 선택해주세요: ");
    let _ = io::stdout().flush();
}

fn print_club_menu() {
    println!("                                                            ");
    println!("                                                            ");
    println!("------------------------------------------------------------");
    println!("           소프트웨어학부 동아리 관리 시스템 - 동아리 관리             ");
    println!("------------------------------------------------------------");
    println!("  1. 동아리 전체 목록 조회            2. 동아리 신규 등록           ");
    println!("  3. 동아리 검색                    4. 동아리 정보 변경           ");
    println!("  5. 동아리 삭제                    6.                        ");
    println!("                                99. 이전 메뉴로 이동            ");
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

fn retriever_club_table(conn:&mut  PooledConn, table: TableList) -> std::result::Result<(), Box<dyn std::error::Error>>{
    println!("\n{}의 전체 목록을 출력합니다.", table.table_name());
    match table {
        TableList::Club => {
            let result: Vec<Club> = conn.query_map(
                format!("SELECT * FROM {}", table.table_name()),
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
        }
        TableList::Professor => {
            let result: Vec<Professor> = conn.query_map(
                format!("SELECT * FROM {}", table.table_name()),
                |(profid, profname, phone)| Professor {
                    profid,
                    profname,
                    phone,
                },
            )?;

            for r in result {
                println!("{} {} {}", r.profid, r.profname, r.phone);
            }
        }
        TableList::Member => {
            let result: Vec<Member> = conn.query_map(
                format!("SELECT * FROM {}", table.table_name()),
                |(
                    studentid,
                    name,
                    dept,
                    gender,
                    birth,
                    phone,
                    joindate,
                    isleader,
                    clubid,                
                ): (String, String, Option<String>, Option<String>, String, Option<String>, String, String, i32)| {
                    Member{
                        studentid,
                        name,
                        dept,
                        gender,
                        birth,
                        phone,
                        joindate,
                        isleader: isleader == "Y", // CHAR('Y')를 bool로 변환
                        clubid,
                    }
                },
            )?;

            for r in result {
                println!("{} {} {} {} {} {} {} {} {}", 
                    r.studentid, 
                    r.name, 
                    r.dept.unwrap_or("NULL".to_string()),
                    r.gender.unwrap_or("NULL".to_string()),
                    r.birth,
                    r.phone.unwrap_or("NULL".to_string()),
                    r.joindate,
                    r.isleader,
                    r.clubid,

                );
            }
        }
    }
    Ok(())
}

fn get_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read line");
    let input = user_input.trim();

    input.to_string()
}

fn prof_manament(conn: &mut PooledConn) {
    loop {
        print_prof_menu();
        let input = get_input();

        match input.as_str() {
            "1" => {
                match retriever_club_table(conn, TableList::Professor) {
                    Ok(_) => {},
                    Err(_) => println!("교수 전체 목록을 조회하는 과정에서 에러가 발생했습니다.")
                }
            },
            "2" => {
                
            },
            "3" => {
                println!("검색할 교수의 이름을 정확하게 입력해주세요: ");
                let _ = io::stdout().flush();

                let prof_name = get_input();
                
            },
            "4" => {
                println!("수정할 교수의 이름을 정확하게 입력해주세요: ");
               

            },
            "5" => {
                println!("삭제할 교수의 이름을 정확하게 입력해주세요: ");

            }
            "99" => {
                break;
            },
            _ => println!("잘못 입력했습니다. 메뉴의 번호를 확인해주세요."),
        }
        
    }
} 

fn mem_management(conn: &mut PooledConn) {
    loop {
        print_mem_menu();
        let input = get_input();

        match input.as_str() {
            "1" => {
                match retriever_club_table(conn, TableList::Member) {
                    Ok(_) => {},
                    Err(_) => println!("동아리원 전체 목록을 조회하는 과정에서 에러가 발생했습니다.")
                }
            },
            "2" => {
                
            },
            "3" => {
                println!("검색할 동아리원의 이름을 정확하게 입력해주세요: ");
                let _ = io::stdout().flush();

                let mem_name = get_input();
                
            },
            "4" => {
                println!("수정할 동아리원의 이름을 정확하게 입력해주세요: ");
               

            },
            "5" => {
                println!("삭제할 동아리원의 이름을 정확하게 입력해주세요: ");

            }
            "99" => {
                break;
            },
            _ => println!("잘못 입력했습니다. 메뉴의 번호를 확인해주세요."),
        }
        
    }
}

fn club_management(conn:&mut  PooledConn) {
    loop {
        print_club_menu();
        let input = get_input();

        match input.as_str() {
            "1" => {
                match retriever_club_table(conn, TableList::Club) {
                    Ok(_) => {},
                    Err(_) => println!("동아리 전체 목록을 조회하는 과정에서 에러가 발생했습니다.")
                }
            },
            "2" => {
                
            },
            "3" => {
                println!("검색할 동아리의 이름을 정확하게 입력해주세요: ");
                let _ = io::stdout().flush();

                let club_name = get_input();
                
            },
            "4" => {
                println!("수정할 동아리의 이름을 정확하게 입력해주세요: ");
                let _ = io::stdout().flush();
                println!("동아리명: ");
                let club_name = get_input();

                println!("동아리위치: ");
                let club_loca = get_input();

                println!("동아리 홈페이지: ");
                let club_hp = get_input();

                println!("동아리 리더: ");
                let club_leader = get_input();

                println!("동아리 지도교수: ");
                let club_prof = get_input();
                
                let result = conn.exec_drop(
                    "update?? into Club(clubName, loacation, homepage, leaderID, professorID) values ( :clubName, :loacation, :homepage, :club_leader, :club_leader)",
                    params!{"clubName"=> club_name, "loacation"=> club_loca, "homepage"=> club_hp, "club_leader"=> club_leader, "professorID" =>club_prof}
                );
                match result {
                    Ok(()) => {println!("신규 동아리 등록이 완료되었습니다.")},
                    Err(_)=>println!("cbnu db insert error"),
                }

            },
            "5" => {
                println!("삭제할 동아리의 이름을 정확하게 입력해주세요: ");
                let _ = io::stdout().flush();

                let club_name = get_input();

            }
            "99" => {
                break;
            },
            _ => println!("잘못 입력했습니다. 메뉴의 번호를 확인해주세요."),
        }
        
    }
}
