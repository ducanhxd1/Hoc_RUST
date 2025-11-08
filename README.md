#  RUST
Ngôn ngữ lập trình RUST

## Chương 2 : 
- use std::io                       => Import thư viện nhập/xuất từ Rust standard library.
- rand::thread_rng().gen_range(...) => Tạo số ngẫu nhiên trong khoảng
- .trim()                           => Xóa ký tự thừa như \n từ chuỗi nhập
- .parse()                          => Chuyển chuỗi thành kiểu dữ liệu cụ thể (u32, f64, ...)
## Chương 3: Variables and Mutability

### 3.1- Biến và tính bất biến (immutable)
- Mặc định trong Rust: biến là bất biến (immutable). 
- mut (là mutable) : dùng mut để tạo biến có thể thay đổi.

- Dùng const cho hằng số 
    Vd: const PI: f64 = 3.1415;

Shadowing là một tính năng cho phép bạn khai báo lại một biến với cùng tên trong cùng một phạm vi (scope), ghi đè giá trị trước đó.

Khi nào nên dùng shadowing?
- Khi bạn muốn giữ biến bất biến (không cần dùng mut)
- Khi cần thay đổi kiểu dữ liệu
- Khi muốn viết mã rõ ràng, dễ theo dõi hơn (ví dụ: qua từng bước chuyển đổi giá trị)

Scope
    Giá trị của variables có thể được xác định tùy theo scope. Scope là một tập hợp nằm trong {}

    let a = 1;
    {
        let a = 2;
        println!("innner: a = {}", a); // = 2
    }
    println!("outer: a = {}", a); // = 1

Return trong scope 
    Có thể return giá trị trong 1 scopce cho 1 biến

    let a = {
        let y = 10;
        let z = 100;

        y+z
    }; 
    println!("a = {}", a);
### 3.2- Kiểu dữ liệu trong RUST
- 
