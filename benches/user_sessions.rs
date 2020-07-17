use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum UserRole {
    Admin,
    User,
    Guest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Session {
    #[serde(with = "serde_bytes")]
    token: Vec<u8>,
    expires: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    role: UserRole,
    session: Option<Session>,
}

fn admin_sample() -> (User, Vec<u8>) {
    let sample = User {
        id: 42,
        name: "Jane Doe".to_string(),
        email: "jdoe@example.com".to_string(),
        role: UserRole::Admin,
        session: Some(Session {
            token: b"a2b08ecd0a0dc594ebccd607033e79262d1fa049a6d44165631b10028f97b611".to_vec(),
            expires: 42424242,
        }),
    };
    let ser = serde_bare::to_vec(&sample).unwrap();
    (sample, ser)
}

fn guest_sample() -> (User, Vec<u8>) {
    let sample = User {
        id: 112,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        role: UserRole::Guest,
        session: None,
    };
    let ser = serde_bare::to_vec(&sample).unwrap();
    (sample, ser)
}

fn serialize_admin(c: &mut Criterion) {
    let (sample, ser) = admin_sample();
    let mut group = c.benchmark_group("serialization");
    group.throughput(Throughput::Bytes(ser.len() as u64));

    let mut buffer: [u8; 128] = [0; 128];
    group.bench_function("serialize admin", |b| {
        b.iter(|| serde_bare::to_writer(&mut buffer[..], &sample).unwrap())
    });
    group.finish();
}

fn deserialize_admin(c: &mut Criterion) {
    let (_, ser) = admin_sample();
    let mut group = c.benchmark_group("deserialization");
    group.throughput(Throughput::Bytes(ser.len() as u64));

    group.bench_function("deserialize admin", |b| {
        b.iter(|| serde_bare::from_slice::<User>(&ser).unwrap())
    });
    group.finish();
}

fn serialize_guest(c: &mut Criterion) {
    let (sample, ser) = guest_sample();
    let mut group = c.benchmark_group("serialization");
    group.throughput(Throughput::Bytes(ser.len() as u64));

    let mut buffer: [u8; 128] = [0; 128];
    group.bench_function("serialize guest", |b| {
        b.iter(|| serde_bare::to_writer(&mut buffer[..], &sample).unwrap())
    });
    group.finish();
}

fn deserialize_guest(c: &mut Criterion) {
    let (_, ser) = guest_sample();
    let mut group = c.benchmark_group("deserialization");
    group.throughput(Throughput::Bytes(ser.len() as u64));

    group.bench_function("deserialize guest", |b| {
        b.iter(|| serde_bare::from_slice::<User>(&ser).unwrap())
    });
    group.finish();
}

criterion_group!(admin, serialize_admin, deserialize_admin);
criterion_group!(guest, serialize_guest, deserialize_guest);
criterion_main!(admin, guest);
