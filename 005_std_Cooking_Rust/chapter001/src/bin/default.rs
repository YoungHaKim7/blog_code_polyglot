#[derive(Default, Debug)]
struct 피자가게Config {
    치즈를_원함: bool,
    올리브_no: i32,
    특별한_메세지: String,
    크러스트_type: CrustType,
}

#[derive(Debug)]
enum CrustType {
    두껍게,
    중간,
    얇게,
}

impl Default for CrustType {
    fn default() -> CrustType {
        CrustType::얇게
    }
}

fn main() {
    let my_default_no: i64 = Default::default();
    println!("i64 default 하면 : {my_default_no}");

    let 피자: 피자가게Config = Default::default();

    println!("내가 원하는 피자(치즈를 원함??) : {}", 피자.치즈를_원함);
    println!("내가 원하는 피자(올리브 갯수) : {}", 피자.올리브_no);
    println!("내가 원하는 피자(특별한 메세지) : {}", 피자.특별한_메세지);
    println!("내가 원하는 피자(크러스트 type) : {:?}", 피자.크러스트_type);

    let 나만의_피자 = 피자가게Config {
        치즈를_원함: true,
        올리브_no: 20,
        특별한_메세지: "무조건 많이~~~".to_string(),
        크러스트_type: CrustType::두껍게,
    };

    println!("~~~~\n다르게 해 보자~~\n");
    println!("나의 피자 스타일 : {:#?}", 나만의_피자);

    let 피자주문_002 = 피자가게Config {
        치즈를_원함: true,
        올리브_no: 20,
        ..Default::default()
    };

    println!("~~~~\n다르게 해 보자~~\n");
    println!("다른 주문 귀찮은거 처리: {:#?}", 피자주문_002);
}
