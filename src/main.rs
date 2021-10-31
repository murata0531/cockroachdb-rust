use app::{
    models::{self, NewUser, User},
    schema,
};
use diesel::prelude::*;
use rayon::prelude::*;

fn main() {
    let concurrency = 100;
    let n = 100000;

    rayon::ThreadPoolBuilder::new().num_threads(concurrency as usize).build_global().unwrap();

    let pool = models::create_db_pool(concurrency);

    (1..=n).into_par_iter().for_each(|_|{
        let conn = pool.get().unwrap();

        let user = conn.transaction(|| {
            diesel::insert_into(schema::users::table)
                .values(&NewUser {
                    name: "hoge".to_owned(),
                })
                .get_result::<User>(&conn)
        });
        // println!("{:?}", user);
    });
}