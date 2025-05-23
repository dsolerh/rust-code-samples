// pub struct PeoplePlugin;

// impl Plugin for PeoplePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, setup)
//             .add_systems(Update, hello_world)
//             .add_systems(Update, print_names)
//             .add_systems(Update, people_with_jobs)
//             .add_systems(Update, people_without_jobs);
//     }
// }

// pub fn hello_world() {
//     println!("Hello World!")
// }

// pub fn setup(mut commands: Commands) {
//     let people_with_job = vec![
//         ("daniel", Job::Doctor),
//         ("karla", Job::FireFighter),
//         ("milber", Job::Lawyer),
//         ("david", Job::Doctor),
//     ];
//     for (person_name, job) in people_with_job {
//         commands.spawn((
//             Person {
//                 name: person_name.to_string(),
//             },
//             Employed { job },
//         ));
//     }

//     let people_without_job = vec!["francis", "robert", "victor"];
//     for person_name in people_without_job {
//         commands.spawn(Person {
//             name: person_name.to_string(),
//         });
//     }
// }

// pub fn print_names(person_query: Query<&Person>) {
//     for person in person_query.iter() {
//         println!("Name: {}", person.name);
//     }
// }

// pub fn people_with_jobs(person_query: Query<(&Person, &Employed), With<Employed>>) {
//     for (person, employed) in person_query.iter() {
//         println!("Name: {} has a job ({:?}).", person.name, employed.job);
//     }
// }

// pub fn people_without_jobs(person_query: Query<&Person, Without<Employed>>) {
//     for person in person_query.iter() {
//         println!("Name: {} does not have a job.", person.name);
//     }
// }

// #[derive(Component)]
// pub struct Person {
//     pub name: String,
// }

// #[derive(Component)]
// pub struct Employed {
//     pub job: Job,
// }

// #[derive(Debug)]
// pub enum Job {
//     Doctor,
//     FireFighter,
//     Lawyer,
// }
