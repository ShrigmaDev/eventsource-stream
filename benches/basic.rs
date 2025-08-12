use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use eventsource_stream::EventStream;
use futures::prelude::*;

fn benchmark_event_stream(c: &mut Criterion) {
    let event_template = r#"event: live
data: {"Ts":1754977913936,"Prices":[1.270,1.770,1.370],"PriceNames":["hd","ad","ha"],"MarketParameters":null,"MarketPeriod":null,"SuperOddsType":"DOUBLECHANCE_PARTICIPANT_RESULT","MessageId":"1790299788:00003:000141","StartTime":"2025-08-23T16:30:00Z","IsTeam":true,"InRunning":false,"BookmakerId":539,"Participant1IsHome":true,"FixtureId":16924188}

"#;

    let mut event_data_vec = Vec::with_capacity(300);
    for i in 0..300 {
        // Create a slightly different event for each iteration to avoid any potential optimizations
        let modified_event = event_template.replace("MessageId\":\"1790299788:00003:000141", 
                                                   &format!("MessageId\":\"1790299788:00003:{:06}", i));
        event_data_vec.push(Ok::<_, ()>(modified_event));
    }

    c.bench_function("parse_300_live_events", |b| {
        b.iter(|| {
            futures::executor::block_on(async {
                let stream = futures::stream::iter(event_data_vec.clone());
                let events = EventStream::new(stream)
                    .try_collect::<Vec<_>>()
                    .await
                    .unwrap();

                black_box(events);
            })
        })
    });
}

criterion_group!(benches, benchmark_event_stream);
criterion_main!(benches);