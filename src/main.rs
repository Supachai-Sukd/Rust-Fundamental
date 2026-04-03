// ==========================================
// RUST FUNDAMENTALS FOR GOLANG DEVELOPERS
// ==========================================

fn main() {
    println!("=== RUST FUNDAMENTALS ===");
    
    println!("\n--- 1. Variables & Mutability ---");
    variables_example();

    println!("\n--- 2. Data Types ---");
    data_types_example();

    println!("\n--- 3. Control Flow ---");
    control_flow_example();

    println!("\n--- 4. Ownership & Borrowing (Rust's Magic) ---");
    ownership_example();

    println!("\n--- 5. Structs & Methods ---");
    struct_example();

    println!("\n--- 6. Enums & Pattern Matching ---");
    enum_and_match_example();

    println!("\n--- 7. Error Handling ---");
    error_handling_example();

    println!("\n--- 8. Traits (like Interfaces) ---");
    traits_example();
}

// ---------------------------------------------------------
// 1. Variables & Mutability
// ---------------------------------------------------------
// ใน Golang:
// var x int = 5 (mutable เสมอ)
// const y = 10 (immutable)
// x := 5 (shorthand, mutable)
//
// ใน Rust:
// ตัวแปรทุกตัวเป็น immutable (เปลี่ยนค่าไม่ได้) โดย default
// ถ้าจะให้เปลี่ยนค่าได้ต้องใส่ keyword `mut` ไว้ข้างหน้าเวลาประกาศ
fn variables_example() {
    let x = 5; // immutable
    println!("x (immutable) = {}", x);
    // x = 6; // Error! ไม่สามารถเปลี่ยนค่าได้เพราะไม่มีคำว่า `mut`

    let mut y = 10; // mutable
    println!("y (mutable initially) = {}", y);
    y = 15;
    println!("y (after mutation) = {}", y);

    // Rust มีกระบวนการ shadowing (การประกาศตัวแปรชื่อเดิมทับของเดิมใน scope ได้)
    // ใน Golang การใช้ := ประกาศตัวแปรรับค่าตัวแปรเดิมซ้ำใน block เดิมจะพัง (no new variables on left side)
    let x = x + 1;
    println!("x (after shadowing) = {}", x);
}

// ---------------------------------------------------------
// 2. Data Types
// ---------------------------------------------------------
// ใน Golang:
// int, int64, float64, string, bool
//
// ใน Rust:
// i32, i64, f64, String หรือ &str (string slices), bool
fn data_types_example() {
    // Rust มักจะ infer type ให้เราแบบอัตโนมัติ (คล้าย := ใน Go)
    let is_active: bool = true;

    // ข้อควรระวัง: String ใน Rust มี 2 ประเภทหลักที่เจอบ่อยมากๆ:
    // - `&str` (String slice): เทียบเท่า string literal ชี้ไปที่หน่วยความจำ (immutable)
    // - `String`: ตัวที่เป็น Object เก็บใน Heap, สามารถขยาย/แก้ไขได้ (เทียบเท่า string ปกติของ Go)
    let greeting_str: &str = "Hello";
    let mut greeting_string: String = String::from("Hello");
    greeting_string.push_str(" World"); // จัดการ string ใน heap ได้

    println!("Boolean: {}", is_active);
    println!("&str: {}", greeting_str);
    println!("String: {}", greeting_string);

    // Tuple ของ Rust สามารถมี types ต่างกันอยู่ในโครงสร้างได้เลย (Go ไม่มี Tuple ตรงๆ แต่มี multi return types)
    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple [0]: {}, [1]: {}", my_tuple.0, my_tuple.1);
}

// ---------------------------------------------------------
// 3. Control Flow
// ---------------------------------------------------------
// ใน Golang:
// if condition { ... }
// for i := 0; i < 5; i++ { ... }
// for condition { ... } // (เอา for มาใช้แทน while)
// switch { ... }
//
// ใน Rust:
// if ไม่ต้องมีวงเล็บเหมือน Go 
// สามารถ return ค่าจาก block if/else ได้ (ใน Go ไม่มี ternary/return แบบนี้ ต้อง assign เอาเอง)
fn control_flow_example() {
    let number = 3;

    // if-else รูปร่างหน้าตาเหมือน Go ครับ
    if number < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    // ใน Rust if คือ expression มันสามารถสาดค่าออกมารับใส่ตัวแปรได้เลย!
    let is_even = if number % 2 == 0 { true } else { false };
    println!("is number {} even? {}", number, is_even);

    // For loop ของ Rust มักจะวนด้วย Iterator (เหมือน for ... range ใน Go)
    // ส่วน while loop ก็มีให้ใช้ตามปกติ 
    println!("For loop (range):");
    for i in 1..=3 {
        println!("i = {}", i);
    }
}

// ---------------------------------------------------------
// 4. Ownership & Borrowing
// ---------------------------------------------------------
// *** นี่คือหัวใจสำคัญของ Rust ที่ทำให้แตกต่างจากภาษาอื่น
// 
// ใน Golang:
// เรามี Garbage Collector (GC) คอยตามกวาดขยะหน่วยความจำที่ไม่ได้ใช้งานให้
//
// ใน Rust:
// ใช้กฏ Ownership หุ้ม memory ไว้ ไม่มี GC
// 1 data จะมีตัวแปรที่ "เป็นเจ้าของ" ได้แค่ 1 อันเท่านั้น หากเราโยนต่อให้คนอื่น สิทธิ์นั้นจะขาด
// ค่าจะถูก "Move" ยกเว้นเราส่งแบ็คอัพไปเป็น Reference แบบ "ยืม" (Borrow) มาใช้ (`&`)
fn ownership_example() {
    let s1 = String::from("hello_ownership");
    
    // ถ้าเรา let s2 = s1; s1 จะตายทันที เอามา println ต่อจะพัง (Compile ไม่ผ่าน, Ownership ถูกย้ายไปที่ s2 แล้ว)
    
    let s2 = &s1; // ยืมแค่อ่าน (เหมือนเราส่ง Pointer &s1 ใน Go ชั่วคราว)

    // ใน Go pointer ใช้แก้ค่าหรือแค่ reference ตรงนี้ก็เหมือนกัน ต่างที่ Rust เช็คจุกจิกเรื่องยืมให้ปลอดภัยมากๆตอน compile
    println!("s1 remains active: {}", s1);
    println!("s2 borrowed from s1: {}", s2);
}

// ---------------------------------------------------------
// 5. Structs & Methods
// ---------------------------------------------------------
// ใน Golang:
// type User struct { Name string }
// func (u *User) GetName() { ... }
//
// ใน Rust:
// รูปร่าง Data Structure คล้ายกัน คือประกาศ struct แล้วเอา method ไปแปะใน block `impl`
struct User {
    username: String,
    active: bool,
}

// implementation logic ทั้งหมดรวมกันอยู่ใน block เดียว ไม่กระจายเหมือนของ Go
impl User {
    // ฟังก์ชันที่รับ &self แปลว่าเป็น Method (ใน Go คือตัว receiver type (u *User))
    fn is_active(&self) -> bool {
        self.active
    }

    // ฟังก์ชันที่ไม่มี &self เรียกว่า Associated Function 
    // เหมือนเวลาทำ Constructor ของ Go (เช้น func NewUser() *User)
    fn new(username: &str) -> User {
        User {
            username: String::from(username),
            active: true,
        }
    }
}

fn struct_example() {
    let user1 = User::new("gopher_learning");
    println!("User: {}, Active?: {}", user1.username, user1.is_active());
}

// ---------------------------------------------------------
// 6. Enums & Pattern Matching
// ---------------------------------------------------------
// ใน Golang:
// ไม่มี Enum ตรงๆ ต้องประยุกต์ const กับ iota แล้วใช้ switch ในการเช็คแต่ละแบบเอาเอง
//
// ใน Rust:
// Enum ของจริง!! แถมทรงพลังที่สุด มันสามารถเก็บข้อมูลลงไปในบางหัวข้อ (Variant) ของ Enum ได้เลย
enum WebEvent {
    PageLoad,
    KeyPress(char),            // มีข้อมูล char แนบไว้ใน state
    Click { x: i64, y: i64 },  // แนบข้อมูล struct พิกัดเม้าส์ได้เลย!
}

fn enum_and_match_example() {
    let event = WebEvent::KeyPress('R');

    // match ของ Rust บังคับต้องดักให้ครบทุกหน้า (Exhaustive) และดึงข้อมูลที่แฝงอยู่ออกมาได้เลย
    // ของ Go switch/case จะมีโอกาสเขียนหลุดได้ไม่ครบเคส หรือถ้าแฝงข้อมูลก็ต้องทำ Struct Interface เล่นแร่แปรธาตุกันเหนื่อย
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::KeyPress(c) => println!("Key '{}' pressed", c),
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}", x, y),
    }
}

// ---------------------------------------------------------
// 7. Error Handling
// ---------------------------------------------------------
// ใน Golang:
// เราถนัดมาก: func DoJob() (string, error)
// และชินชากับการ: if err != nil { return nil, err }
//
// ใน Rust:
// ไม่มีการ return สองตัว (Tuple return pattern ของ Go)
// ให้ค่า Result<T, E> มาเป็น Standard แทน ซึ่งก็คือ Enum ของข้อ 6 (มี 2 อาการคือ Ok(value) / Err(value))
fn read_file_mock() -> Result<String, String> {
    let success = true; // ลองปรับเป็น false จะเห็นภาพ
    
    if success {
        Ok(String::from("data from file"))
    } else {
        Err(String::from("file not found"))
    }
}

fn perform_action() -> Result<(), String> {
    // ใน Rust จะมีเครื่องหมายปริศนา `?` (Question Mark Operator) ต่อท้ายคำสั่งที่พ่น Result
    // การใส่ ? = "เห้ย ถ้าเป็น error (Err) ให้ return ออกจากฟังก์ชันนี้ส่ง error ไปให้คนเรียกเลย"
    // เทียบเท่า if err != nil { return err } ใน Go เดี๊ยะๆ
    let data = read_file_mock()?; 
    println!("Read content was a success: {}", data);
    Ok(()) // () ของ Rust หมายถึง return empty แบบ void (เทียบเท่า error == nil ใน Go)
}

fn error_handling_example() {
    match perform_action() {
        Ok(_) => println!("Action completed without problems."),
        Err(e) => println!("Boom! Error occurred: {}", e),
    }
}

// ---------------------------------------------------------
// 8. Traits (like Interfaces)
// ---------------------------------------------------------
// ใน Golang:
// Duck Typing! เราใช้ type Speaker interface { Speak() string }
// แล้วมี Struct ที่สร้างรับ Speak() เป็นอันถือว่าผ่าน interface อัตโนมัติ (Implicit)
//
// ใน Rust:
// Trait = Interface
// ต้องประกาศแบบจะแจ้งว่า Struct นี้ Implement โลจิกนี้ (Explicit)
trait Speaker {
    fn speak(&self) -> String;
}

struct Dog {
    name: String,
}

// ประกาศเจาะจงลงไปเลยว่าขอ implement trait นี้ให้หมา
impl Speaker for Dog {
    fn speak(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

fn traits_example() {
    let dog = Dog { name: String::from("Buddy") };
    
    // เวลาทำ polymorphism ส่ง interface ใน Golang: func makeNoise(s Speaker)
    // ใน Rust: คือการส่ง parameter รับ &impl Trait ต่างๆ
    fn make_noise(s: &impl Speaker) {
        println!("{}", s.speak());
    }

    make_noise(&dog);
}
