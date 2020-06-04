use salsa::Runtime;

struct Context;

trait HasContext<'cx> {
    fn context(&self) -> &'cx Context;
}

#[salsa::query_group(MyDatabaseStorage)]
trait MyDatabase<'cx>: HasContext<'cx> { }

struct Database<'cx> {
    // runtime: salsa::Runtime<Database<'cx>>,
    context: &'cx Context
}

// impl<'cx> salsa::Database for Database<'cx> {
//     fn salsa_runtime(&self) -> &Runtime<Self> {
//         &self.runtime
//     }
//
//     fn salsa_runtime_mut(&mut self) -> &mut Runtime<Self> {
//         &mut self.runtime
//     }
// }

#[test]
fn test() {
    let context = Context;
    let database = Database {
        // runtime: Runtime::default(),
        context: &context
    };
}
