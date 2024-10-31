# seria_async
Serialized execute async function in non-async function


The current can just execute normal async-function yet, 
in some special scenes, developers need to mark the lifetime for parameters,

```
fn main() {
        // `Type` is specified by developers
        let exec = SingleExecutorType::spawn(func::<Type>::());
        let output: Type = SingleExecutor::run(&exec);

        println!("Executor Output: {"output:#?}");
}

async fn func<Type>() -> Type {
        todo!("do something here")
}

```