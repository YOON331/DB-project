use dotenv::dotenv;
use mysql::prelude::*;
use mysql::*;
use std::env;
use std::io::{self, Write};
use chrono::NaiveDate;

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

struct Project {
    projectid: i32,
    name: String,
    location: Option<String>,
    startdate: Option<String>,
    enddate: Option<String>,
    clubid: i32,
}

struct ProjectParticipation {
    projectid: i32,
    studentid: String,
    pdate: String,
}

struct Budget {
    budgetid: i32,
    r#type: String,
    content: String,
    date: String,
    price: i32,
    studentid: String,
}
struct Post {
    postid: i32,
    content: String,
    title: String,
    date: String,
    studentid: String,
}

struct Comment {
    commentid: i32,
    content: String,
    date: String,
    postid: i32,
    studentid: String,
}

enum TableList {
    Professor,
    Club,
    Member,
    Project,
    ProjectParticipation,
    Post,
    Comment,
    Budget,
}

impl TableList {
    fn table_name(&self) -> &'static str {
        match self {
            TableList::Professor => "Professor",
            TableList::Club => "Club",
            TableList::Member => "Member",
            TableList::Project => "Project",
            TableList::ProjectParticipation => "ProjectParticipation",
            TableList::Budget => "Budget",
            TableList::Post => "Post",
            TableList::Comment => "Comment",
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
                            "4" => {
                                proj_management(&mut conn);
                            }
                            "5" => {
                                post_management(&mut conn);
                            }
                            "6" => {
                                budget_management(&mut conn);
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
    println!("  1. 동아리 관리");
    println!("  2. 지도교수 관리");
    println!("  3. 동아리원 관리");
    println!("  4. 프로젝트 관리");
    println!("  5. 게시글 관리");
    println!("  6. 예산 관리");
    println!("                                99. 시스템 종료               ");
    println!("------------------------------------------------------------");
    print!("이동을 원하는 메뉴를 선택해주세요: ");
    let _ = io::stdout().flush();
}

fn print_post_menu() {
    println!("                                                            ");
    println!("                                                            ");
    println!("------------------------------------------------------------");
    println!("         소프트웨어학부 동아리 관리 시스템 - 게시글 관리               ");
    println!("------------------------------------------------------------");
    println!("  1. 전체 게시글 목록 조회");
    println!("  2. 게시글 등록");
    println!("  3. 게시글 검색");
    println!("  4. 게시글 수정");
    println!("  5. 게시글 삭제 ");
    println!("  6. 댓글 조회 ");
    println!("  7. 댓글 등록 ");
    println!("  8. 댓글 변경 ");
    println!("  9. 댓글 삭제 ");
    println!("                                99. 이전 메뉴로 이동            ");
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
    println!("  1. 동아리원 전체 목록 조회");
    println!("  2. 동아리원 신규 등록");
    println!("  3. 동아리원 검색");
    println!("  4. 동아리원 정보 변경");
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
    println!("  1. 교수 전체 목록 조회");
    println!("  2. 교수 신규 등록");
    println!("  3. 교수 검색");
    println!("  4. 교수 정보 변경");
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
    println!("  1. 동아리 전체 목록 조회");
    println!("  2. 동아리 신규 등록  ");
    println!("  3. 동아리 검색 ");
    println!("  4. 동아리 정보 변경 ");
    println!("  5. 동아리 삭제");
    println!("                                99. 이전 메뉴로 이동            ");
    println!("------------------------------------------------------------");
    print!("이동을 원하는 메뉴를 선택해주세요: ");
    let _ = io::stdout().flush();
}

fn print_proj_menu() {
    println!("                                                            ");
    println!("                                                            ");
    println!("------------------------------------------------------------");
    println!("           소프트웨어학부 동아리 관리 시스템 - 프로젝트 관리             ");
    println!("------------------------------------------------------------");
    println!("  1. 프로젝트 전체 목록 조회");
    println!("  2. 프로젝트 신규 등록  ");
    println!("  3. 프로젝트 검색 ");
    println!("  4. 프로젝트 정보 변경 ");
    println!("  5. 프로젝트 삭제");
    println!("  6. 프로젝트 참여자 목록 조회");
    println!("  7. 프로젝트 참여자 신규 등록");
    println!("  8. 프로젝트 참여자 정보 수정");
    println!("  9. 프로젝트 참여자 정보 삭제");
    println!("                                99. 이전 메뉴로 이동            ");
    println!("------------------------------------------------------------");
    print!("이동을 원하는 메뉴를 선택해주세요: ");
    let _ = io::stdout().flush();
}

fn print_budget_menu() {
    println!("                                                            ");
    println!("                                                            ");
    println!("------------------------------------------------------------");
    println!("            소프트웨어학부 동아리 관리 시스템 - 예산 관리              ");
    println!("------------------------------------------------------------");
    println!("  1. 예산 내역 전체 조회");
    println!("  2. 예산 내역 등록  ");
    println!("  3. 예산 내역 검색 ");
    println!("  4. 예산 내역 변경 ");
    println!("  5. 예산 내역 삭제");
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
            Err(_) => {
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
        TableList::Project => {
            let result: Vec<Project> = conn.query_map(
                format!("SELECT * FROM {}", table.table_name()),
                |(
                    projectid,
                    name,
                    location,
                    startdate,
                    enddate,
                    clubid,                
                ): (i32, String, Option<String>, Option<String>, Option<String>, i32)| {
                    Project{
                        projectid,
                        name,
                        location,
                        startdate,
                        enddate,
                        clubid,  
                    }
                },
            )?;

            for r in result {
                println!("{} {} {} {} {} {}", 
                    r.projectid,
                    r.name,
                    r.location.unwrap_or("NULL".to_string()),
                    r.startdate.unwrap_or("NULL".to_string()),
                    r.enddate.unwrap_or("NULL".to_string()),
                    r.clubid, 
                );
            }
        }
        TableList::ProjectParticipation => {
            let result: Vec<(i32, String, NaiveDate, String, String, String)> = conn.exec_map(
                "SELECT 
                    pp.projectID, 
                    pp.studentID, 
                    pp.participationDate,
                    m.name AS studentName, 
                    c.clubName AS clubName, 
                    p.name AS projectName 
                FROM 
                    ProjectParticipation pp
                JOIN 
                    Member m ON pp.studentID = m.studentID
                JOIN 
                    Project p ON pp.projectID = p.projectID
                JOIN 
                    Club c ON p.clubID = c.clubID",
                (),
                |(projectid, studentid, pdate, student_name, club_name, project_name)| {
                    (projectid, studentid, pdate, student_name, club_name, project_name)
                },
            )?;
        
            for (projectid, studentid, pdate, student_name, club_name, project_name) in result {
                println!(
                    "프로젝트 ID: {}, 프로젝트 이름: {}, 학생 ID: {}, 학생 이름: {}, 동아리 이름: {}, 참여일: {}",
                    projectid, project_name, studentid, student_name, club_name, pdate.to_string()
                );
            }
        }
        TableList::Budget => {
            let result: Vec<Budget> = conn.query_map(
                format!("SELECT * FROM {}", table.table_name()),
                |(
                    budgetid,
                    r#type,
                    content,                    
                    date,
                    price,
                    studentid,               
                ): (i32, String, String, String, i32, String)| {
                    Budget{
                        budgetid,
                        r#type,
                        content,
                        date,
                        price,
                        studentid,  
                    }
                },
            )?;

            for r in result {
                println!("{} {} {} {} {} {}", 
                    r.budgetid,
                    r.r#type,
                    r.content,
                    r.date,
                    r.price,
                    r.studentid, 
                );
            }
        }
        TableList::Post => {
            let result: Vec<Post> = conn.query_map(
                format!("SELECT * FROM {}", table.table_name()),
                |(
                    postid,
                    content,
                    title,                    
                    date,
                    studentid,               
                ): (i32, String, String, String, String)| {
                    Post{
                        postid,
                        content,
                        title,
                        date,
                        studentid,  
                    }
                },
            )?;

            for r in result {
                println!("{} {} {} {} {}", 
                    r.postid,
                    r.content,
                    r.title,
                    r.date,
                    r.studentid, 
                );
            }
        }
        TableList::Comment => {

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

fn post_management(conn: &mut PooledConn) {
    loop {
        print_post_menu();
        let input = get_input();

        match input.as_str() {
            "1" => {
                match retriever_club_table(conn, TableList::Post) {
                    Ok(_) => {},
                    Err(e) => println!("게시글 조회 중 오류 발생: {}", e),
                }
            },
            "2" => {
                match insert_data_table(conn, TableList::Post) {
                    Ok(_) => {},
                    Err(e) => println!("게시글 등록 중 오류 발생: {}", e),
                }
            },
            "3" => {
                match search_data_table(conn, TableList::Post) {
                    Ok(_) => {},
                    Err(e) => println!("게시글 검색 중 오류 발생: {}", e),
                }
            },
            "4" => {
                match update_data_table(conn, TableList::Post) {
                    Ok(_) => {},
                    Err(e) => println!("게시글 수정 중 오류 발생: {}", e),
                }
            },
            "5" => {
                match delete_data_table(conn, TableList::Post) {
                    Ok(_) => {},
                    Err(e) => println!("게시글 삭제 중 오류 발생: {}", e),
                }
            },
            "6" => {
                match retriever_club_table(conn, TableList::Comment) {
                    Ok(_) => {},
                    Err(e) => println!("댓글 조회 중 오류 발생: {}", e),
                }
            },
            "7" => {
                match insert_data_table(conn, TableList::Comment) {
                    Ok(_) => {},
                    Err(e) => println!("댓글 등록 중 오류 발생: {}", e),
                }
            },
            "8" => {
                match update_data_table(conn, TableList::Comment) {
                    Ok(_) => {},
                    Err(e) => println!("댓글 수정 중 오류 발생: {}", e),
                }
            },
            "9" => {
                match delete_data_table(conn, TableList::Comment) {
                    Ok(_) => {},
                    Err(e) => println!("댓글 삭제 중 오류 발생: {}", e),
                }
            },
            "99" => {
                break;
            },
            _ => println!("잘못 입력했습니다. 메뉴의 번호를 확인해주세요."),
        }
    }
}


fn budget_management(conn: &mut PooledConn) {
    loop {
        print_budget_menu();
        let input = get_input();

        match input.as_str() {
            "1" => {
                match retriever_club_table(conn, TableList::Budget) {
                    Ok(_) => {},
                    Err(_) => println!("예산 내역을 조회하는 과정에서 에러가 발생했습니다.")
                }
            },
            "2" => {
                match insert_data_table(conn, TableList::Budget) {
                    Ok(_) => {},
                    Err(_) => println!("예산 내역을 등록하는 과정에서 에러가 발생했습니다.")
                }
            },
            "3" => {
                match search_data_table(conn, TableList::Budget) {
                    Ok(_) => {},
                    Err(_) => println!("예산 내역을 검색하는 과정에서 에러가 발생했습니다.")
                }
            },
            "4" => {
                match update_data_table(conn, TableList::Budget) {
                    Ok(_) => {},
                    Err(_) => println!("예산 내역을 수정하는 과정에서 에러가 발생했습니다.")
                }
            },
            "5" => {
                match delete_data_table(conn, TableList::Budget) {
                    Ok(_) => {},
                    Err(_) => println!("예산 내역을 삭제하는 과정에서 에러가 발생했습니다.")
                }
            }
            "99" => {
                break;
            },
            _ => println!("잘못 입력했습니다. 메뉴의 번호를 확인해주세요."),
        }
        
    }
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
                println!("등록할 교수의 정보를 정확하게 입력해주세요");
                match insert_data_table(conn, TableList::Professor) {
                    Ok(_) => {},
                    Err(_) => println!("교수 정보를 등록하는 과정에서 에러가 발생했습니다.")
                }
            },
            "3" => {
                match search_data_table(conn, TableList::Professor) {
                    Ok(_) => {},
                    Err(e) => println!("검색 중 에러 발생: {}", e),
                }  
            },
            "4" => {
                match update_data_table(conn, TableList::Professor) {
                    Ok(_) => {},
                    Err(e) => println!("수정 중 에러 발생: {}", e),
                }
            },
            "5" => {
                match delete_data_table(conn, TableList::Professor) {
                    Ok(_) => {},
                    Err(e) => println!("삭제 중 에러 발생: {}", e),
                }
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
                println!("신규 등록할 동아리원의 정보를 입력해주세요");
                match insert_data_table(conn, TableList::Member) {
                    Ok(_) => {},
                    Err(_) => println!("동아리원을 등록하는 과정에서 에러가 발생했습니다.")
                }
            },
            "3" => {
                match search_data_table(conn, TableList::Member) {
                    Ok(_) => {},
                    Err(e) => println!("검색 중 에러 발생: {}", e),
                }  
            },
            "4" => {
                match update_data_table(conn, TableList::Member) {
                    Ok(_) => {},
                    Err(e) => println!("수정 중 에러 발생: {}", e),
                }
            },
            "5" => {
                match delete_data_table(conn, TableList::Member) {
                    Ok(_) => {},
                    Err(e) => println!("삭제 중 에러 발생: {}", e),
                }
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
                println!("신규 등록할 동아리 정보를 입력해주세요");
                match insert_data_table(conn, TableList::Club) {
                    Ok(_) => {},
                    Err(_) => println!("동아리 전체 목록을 조회하는 과정에서 에러가 발생했습니다.")
                }
            },
            "3" => {
                match search_data_table(conn, TableList::Club) {
                    Ok(_) => {},
                    Err(e) => println!("검색 중 에러 발생: {}", e),
                }  
            },
            "4" => {
                match update_data_table(conn, TableList::Club) {
                    Ok(_) => {},
                    Err(e) => println!("수정 중 에러 발생: {}", e),
                }
            },
            "5" => {
                match delete_data_table(conn, TableList::Club) {
                    Ok(_) => {},
                    Err(e) => println!("삭제 중 에러 발생: {}", e),
                }
            }
            "99" => {
                break;
            },
            _ => println!("잘못 입력했습니다. 메뉴의 번호를 확인해주세요."),
        }
        
    }
}


fn proj_management(conn: &mut PooledConn) {
    loop {
        print_proj_menu();
        let input = get_input();

        match input.as_str() {
            "1" => {
                match retriever_club_table(conn, TableList::Project) {
                    Ok(_) => {},
                    Err(_) => println!("프로젝트 전체 목록을 조회하는 과정에서 에러가 발생했습니다.")
                }
            },
            "2" => {
                println!("신규 등록할 프로젝트의 정보를 입력해주세요");
                match insert_data_table(conn, TableList::Project) {
                    Ok(_) => {},
                    Err(_) => println!("프로젝트를 등록하는 과정에서 에러가 발생했습니다.")
                }
            },
            "3" => {
                match search_data_table(conn, TableList::Project) {
                    Ok(_) => {},
                    Err(e) => println!("검색 중 에러 발생: {}", e),
                }  
            },
            "4" => {
                match update_data_table(conn, TableList::Project) {
                    Ok(_) => {},
                    Err(e) => println!("수정 중 에러 발생: {}", e),
                }
            },
            "5" => {
                match delete_data_table(conn, TableList::Project) {
                    Ok(_) => {},
                    Err(e) => println!("삭제 중 에러 발생: {}", e),
                }
            }
            "6" => {
                match retriever_club_table(conn, TableList::ProjectParticipation) {
                    Ok(_) => {},
                    Err(e) => println!("조회 중 에러 발생: {}", e),
                }
            }
            "7" => {
                match insert_data_table(conn, TableList::ProjectParticipation) {
                    Ok(_) => {},
                    Err(_) => println!("프로젝트 참여자를 등록하는 과정에서 에러가 발생했습니다.")
                }
            }
            "8" => {
                match update_data_table(conn, TableList::ProjectParticipation) {
                    Ok(_) => {},
                    Err(e) => println!("수정 중 에러 발생: {}", e),
                }
            }
            "9" => {
                match delete_data_table(conn, TableList::ProjectParticipation) {
                    Ok(_) => {},
                    Err(e) => println!("삭제 중 에러 발생: {}", e),
                }
            }
            "99" => {
                break;
            },
            _ => println!("잘못 입력했습니다. 메뉴의 번호를 확인해주세요."),
        }
        
    }
}


fn insert_data_table(conn:&mut  PooledConn, table: TableList) -> std::result::Result<(), Box<dyn std::error::Error>>{
    match table {
        TableList::Club => {
            print!("동아리명: ");
            let _ = io::stdout().flush();
            let club_name = get_input();

            print!("동아리 위치(없는 경우 enter키를 눌러주세요): ");
            let _ = io::stdout().flush();
            let club_location = process_null_input(get_input());

            print!("동아리 홈페이지(없는 경우 enter키를 눌러주세요): ");
            let _ = io::stdout().flush();
            let club_homepage = process_null_input(get_input());

            print!("동아리 리더 학번: ");
            let _ = io::stdout().flush();
            let club_leader = process_null_input(get_input());

            print!("동아리 지도교수 사번: ");
            let _ = io::stdout().flush();
            let club_professor = process_null_input(get_input());

            let result = conn.exec_drop(
                "INSERT INTO Club (clubName, location, homepage, leaderID, professorID) 
                VALUES (:clubname, :location, :homepage, :leaderid, :professorid)",
                params! {
                    "clubname" => club_name,
                    "location" => club_location,
                    "homepage" => club_homepage,
                    "leaderid" => club_leader,
                    "professorid" => club_professor,
                },
            );

            match result {
                Ok(()) => println!("신규 동아리 등록이 완료되었습니다."),
                Err(e) => println!("cbnu db insert error: {}", e),
            }

        }
        TableList::Professor => {
            print!("교수 사번: ");
            let _ = io::stdout().flush();
            let profid = get_input();

            print!("교수 성명: ");
            let _ = io::stdout().flush();
            let profname = get_input();

            print!("교수 전화번호: ");
            let _ = io::stdout().flush();
            let profphone = get_input();
            
            let result = conn.exec_drop(
                "insert into Professor(professorID, name, phone) values ( :id, :name, :phone)",
                params!{"id"=> profid, "name"=> profname, "phone"=> profphone}
            );
            match result {
                Ok(()) => {println!("신규 교수 등록이 완료되었습니다.")},
                Err(_)=>println!("cbnu db insert error"),
            }
        }
        TableList::Member => {
            print!("학번: ");
            let _ = io::stdout().flush();
            let studentid = get_input();
        
            print!("이름: ");
            let _ = io::stdout().flush();
            let name = get_input();
        
            print!("소속 학과: ");
            let _ = io::stdout().flush();
            let dept = get_input();
        
            print!("성별(M/F): ");
            let _ = io::stdout().flush();
            let gender = get_input();
        
            print!("생년월일(2000-01-01): ");
            let _ = io::stdout().flush();
            let birth = get_input();
        
            print!("전화번호(- 없이 입력): ");
            let _ = io::stdout().flush();
            let phone = get_input();
        
            print!("가입일자(2024-01-01): ");
            let _ = io::stdout().flush();
            let joindate = get_input();
        
            print!("임원여부(Y/N): ");
            let _ = io::stdout().flush();
            let isleader = get_input();
        
            print!("가입 동아리(번호 또는 이름 입력): ");
            let _ = io::stdout().flush();
            let joinclub = get_input();
        
            // 가입 동아리 번호 확인
            let club_id: Option<i32> = conn.exec_first(
                "SELECT clubID FROM Club WHERE clubID = :joinclub OR clubName = :joinclub",
                params! { "joinclub" => &joinclub },
            )?;
        
            if club_id.is_none() {
                println!("동아리가 존재하지 않습니다.");
                return Ok(());
            }
            
            // 생년월일 처리
            let birth_date = NaiveDate::parse_from_str(&birth, "%Y-%m-%d")
                .map_err(|_| "유효하지 않은 생년월일 형식입니다")?;

            // 가입일자 처리
            let join_date = NaiveDate::parse_from_str(&joindate, "%Y-%m-%d")
                .map_err(|_| "유효하지 않은 가입일자 형식입니다")?;
                
            // INSERT INTO Member 실행
            let result = conn.exec_drop(
                "INSERT INTO Member (studentID, name, department, gender, birth, phone, joindate, isleader, clubID)
                VALUES (:studentid, :name, :department, :gender, :birth, :phone, :joindate, :isleader, :clubid)",
                params! {
                    "studentid" => studentid,
                    "name" => name,
                    "department" => if dept.to_lowercase() == "null" { None } else { Some(dept) },
                    "gender" => if gender.to_lowercase() == "null" { None } else { Some(gender) },
                    "birth" => birth_date,
                    "phone" => if phone.to_lowercase() == "null" { None } else { Some(phone) },
                    "joindate" => join_date,
                    "isleader" => if isleader.to_lowercase() == "y" { "Y" } else { "N" },
                    "clubid" => club_id.unwrap(),
                },
                
            );
        
            match result {
                Ok(()) => println!("신규 회원 등록이 완료되었습니다."),
                Err(e) => println!("cbnu db insert error: {}", e),
            }
        }        
        TableList::Project => {
            print!("담당 동아리(번호 또는 이름 입력): ");
            let _ = io::stdout().flush();
            let joinclub = get_input();
        
            // 가입 동아리 번호 확인
            let club_id: Option<i32> = conn.exec_first(
                "SELECT clubID FROM Club WHERE clubID = :joinclub OR clubName = :joinclub",
                params! { "joinclub" => &joinclub },
            )?;
        
            if club_id.is_none() {
                println!("동아리가 존재하지 않습니다.");
                return Ok(());
            }
            
            
            print!("프로젝트명: ");
            let _ = io::stdout().flush();
            let name = get_input();
        
            print!("위치: ");
            let _ = io::stdout().flush();
            let location = get_input();
        
            print!("시작일(2024-01-01): ");
            let _ = io::stdout().flush();
            let startdate = get_input();
        
            print!("예상 종료일(2024-01-01): ");
            let _ = io::stdout().flush();
            let enddate = get_input();
        
            let startdate: NaiveDate = NaiveDate::parse_from_str(&startdate, "%Y-%m-%d")
                .map_err(|_| "유효하지 않은 시작일 형식입니다")?;

            let enddate = NaiveDate::parse_from_str(&enddate, "%Y-%m-%d")
                .map_err(|_| "유효하지 않은 종료일 형식입니다")?;
                
            // INSERT INTO Member 실행
            let result = conn.exec_drop(
                "INSERT INTO Project ( name, location, startdate, enddate, clubID)
                VALUES ( :name, :location, :startdate, :enddate, :clubid)",
                params! {
                    "name" => name,
                    "location" => location,
                    "startdate" => startdate,
                    "enddate" => enddate,
                    "clubid" => club_id.unwrap(),
                },
                
            );
            match result {
                Ok(()) => println!("신규 프로젝트 등록이 완료되었습니다."),
                Err(e) => println!("cbnu db insert error: {}", e),
            }
        }
        TableList::ProjectParticipation => {
            print!("참여 동아리원 ID 입력: ");
            let _ = io::stdout().flush();
            let stdid = get_input();
        
            print!("프로젝트 ID 입력: ");
            let _ = io::stdout().flush();
            let projid = get_input();
        
            print!("참여일 (예: 2024-01-01): ");
            let _ = io::stdout().flush();
            let pdate = get_input();
        
            let project_exists: Option<i32> = conn.exec_first(
                "SELECT projectID FROM Project WHERE projectID = :projid",
                params! { "projid" => &projid },
            )?;
        
            if project_exists.is_none() {
                println!("프로젝트가 존재하지 않습니다.");
                return Ok(());
            }
        
            let participation_date: NaiveDate = match NaiveDate::parse_from_str(&pdate, "%Y-%m-%d") {
                Ok(date) => date,
                Err(_) => {
                    println!("유효하지 않은 참여일 형식입니다. YYYY-MM-DD 형식이어야 합니다.");
                    return Ok(());
                }
            };
        
            let result = conn.exec_drop(
                "INSERT INTO ProjectParticipation (projectID, studentID, participationDate)
                 VALUES (:projectid, :studentid, :participationdate)",
                params! {
                    "projectid" => projid,
                    "studentid" => stdid,
                    "participationdate" => participation_date,
                },
            );
        
            match result {
                Ok(()) => println!("프로젝트 참여 등록이 완료되었습니다."),
                Err(e) => println!("DB 삽입 오류: {}", e),
            }
        }
        TableList::Budget => {
            print!("사용 유형: ");
            let _ = io::stdout().flush();
            let btype = get_input();

            print!("사용 내역: ");
            let _ = io::stdout().flush();
            let content = get_input();

            print!("사용일자 (YYYY-MM-DD): ");
            let _ = io::stdout().flush();
            let date = get_input();

            // 날짜 형식 검증
            let parsed_date = match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => {
                    println!("잘못된 날짜 형식입니다. YYYY-MM-DD 형식이어야 합니다.");
                    return Ok(());
                }
            };

            print!("사용금액: ");
            let _ = io::stdout().flush();
            let price = get_input();

            print!("사용 동아리(담당자): ");
            let _ = io::stdout().flush();
            let studentid = get_input();

            let result = conn.exec_drop(
                "insert into Budget(type, content, date, price, studentID) 
                values (:btype, :content, :date, :price, :studentid)",
                params! {
                    "btype" => btype,
                    "content" => content,
                    "date" => parsed_date, 
                    "price" => price,
                    "studentid" => studentid
                }
            );

            match result {
                Ok(()) => {
                    println!("신규 예산 내역 등록이 완료되었습니다.");
                }
                Err(_) => println!("cbnu db insert error"),
            }
        }
        TableList::Post => {
            print!("게시글 제목: ");
            let _ = io::stdout().flush();
            let title = get_input();

            print!("게시글 내용: ");
            let _ = io::stdout().flush();
            let content = get_input();

            print!("작성자 ID: ");
            let _ = io::stdout().flush();
            let student_id = get_input();

            let date = chrono::Local::now().date_naive();

            let result = conn.exec_drop(
                "INSERT INTO Post (title, content, date, studentID) VALUES (:title, :content, :date, :studentid)",
                params! {
                    "title" => title,
                    "content" => content,
                    "date" => date,
                    "studentid" => student_id,
                },
            );

            match result {
                Ok(()) => {
                    println!("게시글 등록이 완료되었습니다.");
                }
                Err(_) => println!("cbnu db insert error"),
            }
        }
        TableList::Comment => {

        }
    }
    Ok(())
}

fn process_null_input(input: String) -> Option<String> {
    if input.to_lowercase() == "null" || input.is_empty() {
        None
    } else {
        Some(input)
    }
}


fn update_data_table(conn: &mut PooledConn, table: TableList) -> std::result::Result<(), Box<dyn std::error::Error>> {
    match table {
        TableList::Club => {
            print!("수정할 동아리의 이름을 입력해주세요: ");
            let _ = io::stdout().flush();
            let club_name = get_input();

            let existing: Option<(i32, String, Option<String>, Option<String>, Option<String>, Option<String>)> = conn.exec_first(
                "SELECT clubID, clubName, location, homepage, leaderID, professorID FROM Club WHERE clubName = :clubname",
                params! { "clubname" => &club_name },
            )?;

            if let Some((clubid, clubname, location, homepage, leaderid, professorid)) = existing {
                let club = Club {
                    clubid,
                    clubname,
                    location,
                    homepage,
                    leaderid,
                    professorid,
                };

                print!("새로운 동아리 위치 (기존: {}): ", club.location.clone().unwrap_or("NULL".to_string()));
                let _ = io::stdout().flush();
                let new_location = process_input_or_default(get_input(), club.location);

                print!("새로운 동아리 홈페이지 (기존: {}): ", club.homepage.clone().unwrap_or("NULL".to_string()));
                let _ = io::stdout().flush();
                let new_homepage = process_input_or_default(get_input(), club.homepage);

                print!("새로운 동아리 리더 학번 (기존: {}): ", club.leaderid.clone().unwrap_or("NULL".to_string()));
                let _ = io::stdout().flush();
                let new_leader = process_input_or_default(get_input(), club.leaderid);

                print!("새로운 동아리 지도교수 학번 (기존: {}): ", club.professorid.clone().unwrap_or("NULL".to_string()));
                let _ = io::stdout().flush();
                let new_prof = process_input_or_default(get_input(), club.professorid);

                // 데이터 업데이트
                conn.exec_drop(
                    "UPDATE Club 
                     SET location = :location, homepage = :homepage, leaderID = :leader, professorID = :professor
                     WHERE clubName = :clubname",
                    params! {
                        "location" => new_location,
                        "homepage" => new_homepage,
                        "leader" => new_leader,
                        "professor" => new_prof,
                        "clubname" => &club_name,
                    },
                )?;
                println!("동아리 정보가 성공적으로 수정되었습니다.");
            } else {
                println!("존재하지 않는 동아리입니다.");
            }
        },
        TableList::Professor => {
            print!("수정할 교수의 사번을 입력해주세요: ");
            let _ = io::stdout().flush();
            let prof_id = get_input();

            // 기존 데이터 가져오기
            let existing: Option<(String, String, String)> = conn.exec_first(
                "SELECT professorID, name, phone FROM Professor WHERE professorID = :profid",
                params! { "profid" => &prof_id },
            )?;

            if let Some((prof_id, prof_name, phone)) = existing {

                print!("새로운 전화번호 (기존: {}): ", phone);
                let _ = io::stdout().flush();
                let new_phone = process_input_or_default(get_input(), Some(phone));

                conn.exec_drop(
                    "UPDATE Professor SET phone = :phone WHERE professorID = :profid",
                    params! {
                        "phone" => new_phone,
                        "profid" => &prof_id,
                    },
                )?;
                println!("교수 정보가 수정되었습니다.");
            } else {
                println!("존재하지 않는 교수입니다.");
            }
        },  
        TableList::Member => {
            print!("수정할 동아리원의 이름을 입력해주세요: ");
            let _ = io::stdout().flush();
            let mem_name = get_input();
        
            // 기존 데이터 가져오기
            let existing: Option<(String, String, Option<String>, Option<String>, Option<String>)> = conn.exec_first(
                "SELECT studentID, name, phone, department, isleader FROM Member WHERE name = :memname",
                params! { "memname" => &mem_name },
            )?;
        
            if let Some((student_id, name, phone, department, isleader)) = existing {        
                print!("새로운 전화번호 (기존: {}): ", phone.clone().unwrap_or("NULL".to_string()));
                let _ = io::stdout().flush();
                let new_phone = process_input_or_default(get_input(), phone);
        
                print!("새로운 학과 (기존: {}): ", department.clone().unwrap_or("NULL".to_string()));
                let _ = io::stdout().flush();
                let new_dept = process_input_or_default(get_input(), department);
        
                print!("새로운 임원여부 (기존: {}): ", isleader.clone().unwrap_or("NULL".to_string()));
                let _ = io::stdout().flush();
                let new_leader = process_input_or_default(get_input(), isleader);
        
                conn.exec_drop(
                    "UPDATE Member SET phone = :phone, department = :department, isleader = :isleader WHERE name = :memname",
                    params! {
                        "phone" => new_phone,
                        "department" => new_dept,
                        "isleader" => new_leader,
                        "memname" => &mem_name,
                    },
                )?;
                println!("동아리원 정보가 수정되었습니다.");
            } else {
                println!("존재하지 않는 동아리원입니다.");
            }
        }   
        TableList::Project => {
            print!("수정할 프로젝트의 번호를 입력해주세요: ");
            let _ = io::stdout().flush();
            let proj_id = get_input();
        
            // 기존 데이터 가져오기
            let existing: Option<(i32, String, Option<String>, Option<NaiveDate>, Option<NaiveDate>, i32)> = conn.exec_first(
                "SELECT projectID, name, location, startdate, enddate, clubID FROM Project WHERE projectID = :projid",
                params! { "projid" => &proj_id },
            )?;
        
            if let Some((proj_id, name, location, _, enddate, clubid)) = existing {
        
                print!("새로운 프로젝트 이름 (기존: {}): ", name);
                let _ = io::stdout().flush();
                let new_name = process_input_or_default(get_input(), Some(name));
        
                print!(
                    "새로운 프로젝트 위치 (기존: {}): ",
                    location.clone().unwrap_or("NULL".to_string())
                );
                let _ = io::stdout().flush();
                let new_location = process_input_or_default(get_input(), location);
        
                print!(
                    "새로운 프로젝트 예상 종료일 (기존: {}): ",
                    enddate.map_or("NULL".to_string(), |d| d.to_string())
                );
                let _ = io::stdout().flush();
                let new_enddate = process_input_or_default(get_input(), enddate.map(|d| d.to_string()));
        
                // 업데이트 쿼리 실행
                conn.exec_drop(
                    "UPDATE Project 
                     SET name = :name, location = :location, enddate = :enddate 
                     WHERE projectID = :projid",
                    params! {
                        "name" => new_name,
                        "location" => new_location,
                        "enddate" => new_enddate,
                        "projid" => &proj_id,
                    },
                )?;
                println!("프로젝트 정보가 성공적으로 수정되었습니다.");
            } else {
                println!("존재하지 않는 프로젝트입니다.");
            }
        }       
        TableList::ProjectParticipation => {
            print!("수정할 프로젝트 참여자의 학번을 입력해주세요: ");
            let _ = io::stdout().flush();
            let stdid = get_input();
        
            // 기존 데이터 가져오기
            let existing: Option<(i32, String, Option<NaiveDate>)> = conn.exec_first(
                "SELECT projectID, studentID, participationDate FROM ProjectParticipation WHERE studentID = :stdid",
                params! { "stdid" => &stdid },
            )?;
        
            if let Some((project_id, student_id, participation_date)) = existing {
                print!(
                    "새로운 참여일을 입력해주세요 (기존: {}): ",
                    participation_date.map_or("NULL".to_string(), |date| date.to_string())
                );
                let _ = io::stdout().flush();
                let new_date = get_input();
        
                let new_date = if new_date.trim().is_empty() {
                    participation_date
                } else {
                    Some(NaiveDate::parse_from_str(&new_date, "%Y-%m-%d").unwrap_or_else(|_| {
                        panic!("날짜 형식이 잘못되었습니다. YYYY-MM-DD 형식이어야 합니다.");
                    }))
                };
        
                // ProjectParticipation 테이블 업데이트
                conn.exec_drop(
                    "UPDATE ProjectParticipation SET participationDate = :participationdate WHERE studentID = :studentid",
                    params! {
                        "participationdate" => new_date,
                        "studentid" => student_id,
                    },
                )?;
        
                println!("프로젝트 참여자의 정보가 성공적으로 수정되었습니다.");
            } else {
                println!("존재하지 않는 프로젝트 참여자입니다.");
            }
        }    
        TableList::Budget => {
            print!("수정할 예산의 번호를 입력해주세요: ");
            let _ = io::stdout().flush();
            let id = get_input();
        
            // 기존 데이터 가져오기
            let existing: Option<(i32, String, String, NaiveDate, i32, String)> = conn.exec_first(
                "SELECT budgetID, type, content, date, price, studentID FROM Budget WHERE budgetID = :id",
                params! { "id" => &id },
            )?;
        
            if let Some((budgetid, btype, content, date, price, studentid)) = existing {
                print!(
                    "사용내역을 입력해주세요 (기존: {}): ",
                    content
                );
                let _ = io::stdout().flush();
                let new_content = get_input();
        
                print!(
                    "사용일자를 입력해주세요 (기존: {}): ",
                    date
                );
                let _ = io::stdout().flush();
                let new_date_input = get_input();
        
                print!(
                    "사용금액을 입력해주세요 (기존: {}): ",
                    price
                );
                let _ = io::stdout().flush();
                let new_price_input = get_input();
        
                // 새로운 사용일자 파싱
                let new_date = if new_date_input.trim().is_empty() {
                    date // 기존 값 유지
                } else {
                    NaiveDate::parse_from_str(&new_date_input, "%Y-%m-%d").unwrap_or_else(|_| {
                        panic!("날짜 형식이 잘못되었습니다. YYYY-MM-DD 형식이어야 합니다.");
                    })
                };
        
                // 새로운 사용금액 처리
                let new_price = if new_price_input.trim().is_empty() {
                    price // 기존 값 유지
                } else {
                    new_price_input.trim().parse::<i32>().unwrap_or_else(|_| {
                        panic!("금액은 숫자로 입력해야 합니다.");
                    })
                };
        
                // 새로운 사용내역 처리
                let final_content = if new_content.trim().is_empty() {
                    content.clone() // 기존 값 유지
                } else {
                    new_content
                };
        
                // Budget 테이블 업데이트
                conn.exec_drop(
                    "UPDATE Budget 
                     SET content = :content, date = :date, price = :price 
                     WHERE budgetID = :id",
                    params! {
                        "content" => final_content,
                        "date" => new_date,
                        "price" => new_price,
                        "id" => budgetid,
                    },
                )?;
        
                println!("예산 정보가 성공적으로 수정되었습니다.");
            } else {
                println!("존재하지 않는 예산 번호입니다.");
            }
        }     
        TableList::Post => {
            print!("수정할 게시글 ID를 입력해주세요: ");
            let _ = io::stdout().flush();
            let post_id = get_input();
        
            print!("새로운 제목 (기존 값 유지하려면 엔터): ");
            let _ = io::stdout().flush();
            let new_title = get_input();
        
            print!("새로운 내용 (기존 값 유지하려면 엔터): ");
            let _ = io::stdout().flush();
            let new_content = get_input();
        
            conn.exec_drop(
                "UPDATE Post SET title = COALESCE(NULLIF(:title, ''), title), content = COALESCE(NULLIF(:content, ''), content) WHERE postID = :postid",
                params! {
                    "title" => new_title,
                    "content" => new_content,
                    "postid" => post_id,
                },
            )?;
            println!("게시글이 수정되었습니다.");
        }            
        TableList::Comment => {
            
        }                
    }
    Ok(())
}

fn search_data_table(conn: &mut PooledConn, table: TableList) -> std::result::Result<(), Box<dyn std::error::Error>>{
    match table {
        TableList::Club => {
            print!("검색할 동아리 이름을 입력해주세요: ");
            let _ = io::stdout().flush();
            let club_name = get_input();

            let result: Vec<Club> = conn.exec_map(
                "SELECT * FROM Club WHERE clubName LIKE :clubname",
                params! { "clubname" => format!("%{}%", club_name) },
                |(clubid, clubname, location, homepage, leaderid, professorid)| Club {
                    clubid,
                    clubname,
                    location,
                    homepage,
                    leaderid,
                    professorid,
                },
            )?;

            if result.is_empty() {
                println!("검색 결과가 없습니다.");
            } else {
                for club in result {
                    println!(
                        "{} {} {} {} {} {}",
                        club.clubid,
                        club.clubname,
                        club.location.unwrap_or("NULL".to_string()),
                        club.homepage.unwrap_or("NULL".to_string()),
                        club.leaderid.unwrap_or("NULL".to_string()),
                        club.professorid.unwrap_or("NULL".to_string()),
                    );
                }
            }
        },
        TableList::Professor => {
            print!("검색할 교수 이름을 입력해주세요: ");
            let _ = io::stdout().flush();
            let prof_name = get_input();

            let result: Vec<Professor> = conn.exec_map(
                "SELECT * FROM Professor WHERE name LIKE :profname",
                params! { "profname" => format!("%{}%", prof_name) },
                |(profid, profname, phone)| Professor {
                    profid,
                    profname,
                    phone,
                },
            )?;

            if result.is_empty() {
                println!("검색 결과가 없습니다.");
            } else {
                for prof in result {
                    println!("{} {} {}", prof.profid, prof.profname, prof.phone);
                }
            }
        },
        TableList::Member => {
            print!("검색할 동아리원 이름을 입력해주세요: ");
            let _ = io::stdout().flush();
            let mem_name = get_input();

            let result: Vec<Member> = conn.exec_map(
                "SELECT * FROM Member WHERE name LIKE :memname",
                params! { "memname" => format!("%{}%", mem_name) },
                |(studentid, name, department, gender, birth, phone, joindate, isleader, clubid): (
                    String,           // studentID
                    String,           // name
                    Option<String>,   // department
                    Option<String>,   // gender
                    NaiveDate,        // birth (Date 타입)
                    Option<String>,   // phone
                    NaiveDate,        // joindate (Date 타입)
                    String,           // isleader ("Y"/"N")
                    i32,              // clubID
                )| {
                    Member {
                        studentid,
                        name,
                        dept: department,
                        gender,
                        birth: birth.to_string(), // NaiveDate를 String으로 변환
                        phone,
                        joindate: joindate.to_string(), // NaiveDate를 String으로 변환
                        isleader: isleader == "Y", // CHAR('Y')를 bool로 변환
                        clubid,
                    }
                },
            )?;         

            if result.is_empty() {
                println!("검색 결과가 없습니다.");
            } else {
                for mem in result {
                    println!(
                        "{} {} {} {} {} {} {} {} {}",
                        mem.studentid,
                        mem.name,
                        mem.dept.unwrap_or("NULL".to_string()),
                        mem.gender.unwrap_or("NULL".to_string()),
                        mem.birth,
                        mem.phone.unwrap_or("NULL".to_string()),
                        mem.joindate,
                        mem.isleader,
                        mem.clubid,
                    );
                }
            }
        }
        TableList::Project => {
            print!("검색할 프로젝트의 이름을 입력해주세요: ");
            let _ = io::stdout().flush();
            let proj_name = get_input();

            let result: Vec<Project> = conn.exec_map(
                "SELECT * FROM Project WHERE name LIKE :proj_name",
                params! { "proj_name" => format!("%{}%", proj_name) },
                |(projectid, name, location, startdate, enddate, clubid): (
                    i32,
                    String,
                    Option<String>,
                    Option<NaiveDate>,
                    Option<NaiveDate>,
                    i32,
                )| {
                    Project {
                        projectid,
                        name,
                        location,
                        startdate: startdate.map(|d| d.to_string()), // NaiveDate를 String으로 변환
                        enddate: enddate.map(|d| d.to_string()),       // NaiveDate를 String으로 변환
                        clubid,
                    }
                },
            )?;

            if result.is_empty() {
                println!("검색 결과가 없습니다.");
            } else {
                for proj in result {
                    println!(
                        "{} {} {} {} {} {}",
                        proj.projectid,
                        proj.name,
                        proj.location.unwrap_or("NULL".to_string()),
                        proj.startdate.unwrap_or("NULL".to_string()),
                        proj.enddate.unwrap_or("NULL".to_string()),
                        proj.clubid,
                    );
                }
            }
        }
        TableList::ProjectParticipation => {
            
        }
        TableList::Budget => {
            print!("검색할 예산 사용 내역을 입력해주세요: ");
            let _ = io::stdout().flush();
            let content = get_input();
        
            // 예산 사용 내역 검색
            let result: Vec<Budget> = conn.exec_map(
                "SELECT budgetID, type, content, date, price, studentID 
                    FROM Budget 
                    WHERE content LIKE :content",
                params! { "content" => format!("%{}%", content) },
                |(budgetid, r#type, content, date, price, studentid): (
                    i32,
                    String,
                    String,
                    NaiveDate,
                    i32,
                    String,
                )| {
                    Budget {
                        budgetid,
                        r#type,
                        content,
                        date: date.format("%Y-%m-%d").to_string(),
                        price,
                        studentid,  
                }
            },
            )?;
        
            if result.is_empty() {
                println!("검색 결과가 없습니다.");
            } else {
                println!("검색 결과:");
                for b in result {
                    println!(
                        "예산 ID: {}, 유형: {}, 내역: {}, 날짜: {}, 금액: {}, 담당자: {}",
                        b.budgetid,
                        b.r#type,
                        b.content,
                        b.date,
                        b.price,
                        b.studentid
                    );
                }
            }   
        }
        TableList::Post => {
            print!("검색할 게시글 제목을 입력해주세요: ");
            let _ = io::stdout().flush();
            let title = get_input();
        
            let result: Vec<(i32, String, String, NaiveDate, String)> = conn.exec_map(
                "SELECT postID, title, content, date, studentID FROM Post WHERE title LIKE :title",
                params! { "title" => format!("%{}%", title) },
                |(post_id, title, content, date, student_id)| (post_id, title, content, date, student_id),
            )?;

        
            if result.is_empty() {
                println!("검색 결과가 없습니다.");
            } else {
                println!("검색 결과:");
                for (post_id, title, content, date, student_id) in result {
                    println!(
                        "게시글 ID: {}, 제목: {}, 내용: {}, 작성일: {}, 작성자 ID: {}",
                        post_id, title, content, date, student_id
                    );
                }
            }
            
        }
        TableList::Comment => {
            
        }
    }
    Ok(())
}

fn delete_data_table(conn: &mut PooledConn, table: TableList) -> std::result::Result<(), Box<dyn std::error::Error>> {
    match table {
        TableList::Club => {
            print!("삭제할 동아리명을 입력해주세요: ");
            let _ = io::stdout().flush();
            let club_name = get_input();

            let delete_result = conn.exec_drop(
                "DELETE FROM Club WHERE clubName = :clubname",
                params! { "clubname" => &club_name },
            );

            match delete_result {
                Ok(_) => println!("동아리가 성공적으로 삭제되었습니다."),
                Err(e) => println!("동아리 삭제 중 오류 발생: {}", e),
            }
        }
        TableList::Professor => {
            print!("삭제할 교수의 사번을 입력해주세요: ");
            let _ = io::stdout().flush();
            let prof_id = get_input();

            let delete_result = conn.exec_drop(
                "DELETE FROM Professor WHERE professorID = :profid",
                params! { "profid" => &prof_id },
            );

            match delete_result {
                Ok(_) => println!("교수가 성공적으로 삭제되었습니다."),
                Err(e) => println!("교수 삭제 중 오류 발생: {}", e),
            }
        }
        TableList::Member => {
            print!("삭제할 동아리원의 학번을 입력해주세요: ");
            let _ = io::stdout().flush();
            let student_id = get_input();

            let delete_result = conn.exec_drop(
                "DELETE FROM Member WHERE studentID = :studentid",
                params! { "studentid" => &student_id },
            );

            match delete_result {
                Ok(_) => println!("동아리원이 성공적으로 삭제되었습니다."),
                Err(e) => println!("동아리원 삭제 중 오류 발생: {}", e),
            }
        }
        TableList::Project => {
            print!("삭제할 프로젝트의 번호를 입력해주세요: ");
            let _ = io::stdout().flush();
            let project_id = get_input();

            let delete_result = conn.exec_drop(
                "DELETE FROM Project WHERE projectID = :projectid",
                params! { "projectid" => &project_id },
            );

            match delete_result {
                Ok(_) => println!("프로젝트가 성공적으로 삭제되었습니다."),
                Err(e) => println!("프로젝트 삭제 중 오류 발생: {}", e),
            }
        }
        TableList::ProjectParticipation => {
            print!("삭제할 프로젝트의 번호를 입력해주세요: ");
            let _ = io::stdout().flush();
            let project_id = get_input();
        
            let project_exists: Option<i32> = conn.exec_first(
                "SELECT projectID FROM Project WHERE projectID = :projectid",
                params! { "projectid" => &project_id },
            )?;
        
            if project_exists.is_none() {
                println!("존재하지 않는 프로젝트입니다.");
                return Ok(());
            }
        
            print!("삭제할 프로젝트 참여자의 학번을 입력해주세요: ");
            let _ = io::stdout().flush();
            let std_id = get_input();
        
            let student_exists: Option<String> = conn.exec_first(
                "SELECT studentID FROM Member WHERE studentID = :stdid",
                params! { "stdid" => &std_id },
            )?;
        
            if student_exists.is_none() {
                println!("존재하지 않는 학생입니다.");
                return Ok(());
            }
        
            // 해당 프로젝트의 참여자로 등록 여부 확인
            let participation_exists: Option<i32> = conn.exec_first(
                "SELECT projectID FROM ProjectParticipation WHERE studentID = :stdid AND projectID = :projectid",
                params! { 
                    "stdid" => &std_id,
                    "projectid" => &project_id
                },
            )?;
        
            if participation_exists.is_none() {
                println!("해당 프로젝트에 등록되지 않은 학생입니다.");
                return Ok(());
            }
        
            let delete_result = conn.exec_drop(
                "DELETE FROM ProjectParticipation WHERE studentID = :stdid AND projectID = :projectid",
                params! { 
                    "stdid" => &std_id,
                    "projectid" => &project_id
                },
            );
        
            match delete_result {
                Ok(_) => println!("프로젝트 참여자 정보가 성공적으로 삭제되었습니다."),
                Err(e) => println!("프로젝트 참여자 정보 삭제 중 오류 발생: {}", e),
            }
        }     
        TableList::Budget => {
            print!("삭제할 예산의 번호를 입력해주세요: ");
            let _ = io::stdout().flush();
            let id = get_input();

            let delete_result = conn.exec_drop(
                "DELETE FROM Budget WHERE budgetID = :id",
                params! { "id" => &id },
            );

            match delete_result {
                Ok(_) => println!("선택한 예산 내역이 성공적으로 삭제되었습니다."),
                Err(e) => println!("예산 내역 삭제 중 오류 발생: {}", e),
            }
        }      
        TableList::Post => {
            print!("삭제할 게시글의 번호를 입력해주세요: ");
            let _ = io::stdout().flush();
            let id = get_input();

            let delete_result = conn.exec_drop(
                "DELETE FROM Post WHERE postID = :id",
                params! { "id" => &id },
            );

            match delete_result {
                Ok(_) => println!("선택한 게시글이 성공적으로 삭제되었습니다."),
                Err(e) => println!("게시글 삭제 중 오류 발생: {}", e),
            }
        }
        TableList::Comment => {
            
        }
    }
    Ok(())
}

fn process_input_or_default(input: String, default: Option<String>) -> Option<String> {
    if input.trim().is_empty() {
        default
    } else {
        Some(input)
    }
}