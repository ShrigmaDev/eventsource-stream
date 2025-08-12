use criterion::{black_box, criterion_group, criterion_main, Criterion};
use eventsource_stream::{EventBuilder, parse_event};

fn criterion_benchmark(c: &mut Criterion) {
    // Create a benchmark group for parse_event
    let mut group = c.benchmark_group("parse_event");

    // Simple event with just data
    let simple_event = "data: Hello, world!\n\n";
    group.bench_function("simple_event", |b| {
        b.iter(|| {
            let mut buffer = String::from(black_box(simple_event));
            let mut builder = Default::default();
            parse_event::<()>(&mut buffer, &mut builder)
        })
    });

    // Event with multiple fields
    let complex_event = "event: update\ndata: {\"status\": \"success\"}\nid: 1\n\n";
    group.bench_function("complex_event", |b| {
        b.iter(|| {
            let mut buffer = String::from(black_box(complex_event));
            let mut builder = Default::default();
            parse_event::<()>(&mut buffer, &mut builder)
        })
    });

    // Multiple events in one buffer
    let multiple_events = "data: First event\n\ndata: Second event\n\n";
    group.bench_function("multiple_events", |b| {
        b.iter(|| {
            let mut buffer = String::from(black_box(multiple_events));
            let mut builder = Default::default();
            // Parse first event
            let _ = parse_event::<()>(&mut buffer, &mut builder);
            // Parse second event
            parse_event::<()>(&mut buffer, &mut builder)
        })
    });

    // Event with comments
    let event_with_comments = ": This is a comment\ndata: Event with comment\n\n";
    group.bench_function("event_with_comments", |b| {
        b.iter(|| {
            let mut buffer = String::from(black_box(event_with_comments));
            let mut builder = Default::default();
            parse_event::<()>(&mut buffer, &mut builder)
        })
    });

    // Incomplete event (should return None)
    let incomplete_event = "data: Incomplete event\n";
    group.bench_function("incomplete_event", |b| {
        b.iter(|| {
            let mut buffer = String::from(black_box(incomplete_event));
            let mut builder = Default::default();
            parse_event::<()>(&mut buffer, &mut builder)
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);