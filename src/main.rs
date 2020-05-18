use tokio::runtime::Runtime;
use async_trait::async_trait;

#[async_trait]
trait Converter {
    async fn process(&mut self);
}

struct ConverterA {}

#[async_trait]
impl Converter for ConverterA {
    async fn process(&mut self) {
        println!("process for ConverterA");
        //<Self as Converter>::static_common_func();
    }
}

struct ConverterB {}

#[async_trait]
impl Converter for ConverterB {
    async fn process(&mut self) {
        println!("process for ConverterB");
        //Self::static_common_func();
    }
}

fn main() {

    let condition = "A";
    let converter: Box<dyn Converter>;

    if condition == "A" {
        converter = Box::new(ConverterA {});
    } else {
        converter = Box::new(ConverterB {});
    }

    let mut rt = Runtime::new().unwrap();

    rt.block_on(async move {
            tokio::spawn(async move {
                converter.process().await;
            });
    });

    // rt.block_on(async move {
    //     converter.run().await;
    // });
}
