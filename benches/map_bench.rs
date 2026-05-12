use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

use map_benchmarks::maps::Finder;
use map_benchmarks::maps::lmv1::LinearMap;
use map_benchmarks::maps::lmv2::LinearMapV2;

use map_benchmarks::maps::lmv3::LinearMapV3;
use map_benchmarks::{sample_data_fm, sample_data_fml};

fn build_fw_log_kv() -> Vec<Vec<(String, String)>> {
    sample_data_fm()
        .into_iter()
        .map(|d| d.to_kv_vec())
        .collect()
}

fn build_fw_log_expanded_kv() -> Vec<Vec<(String, String)>> {
    sample_data_fml()
        .into_iter()
        .map(|d| d.to_kv_vec())
        .collect()
}

fn leak_dataset_as_static(logs: &[Vec<(String, String)>]) -> Vec<Vec<(&'static str, String)>> {
    logs.iter()
        .map(|log| {
            log.iter()
                .map(|(k, v)| {
                    let k_static: &'static str = Box::leak(k.clone().into_boxed_str());
                    (k_static, v.clone())
                })
                .collect()
        })
        .collect()
}

fn bench_linear_map(c: &mut Criterion, group_name: &str, logs: Vec<Vec<(String, String)>>) {
    let insert_name = format!("{group_name}/linear_map/insert");
    let find_name = format!("{group_name}/linear_map/find_hit");

    c.bench_function(&insert_name, |b| {
        b.iter_batched(
            || logs.clone(),
            |logs| {
                let mut map = LinearMap::new();

                for log in logs {
                    for (k, v) in log {
                        map.push(k, v.as_str());
                    }
                }

                black_box(map);
            },
            BatchSize::SmallInput,
        );
    });

    let mut map = LinearMap::new();
    let mut hit_keys = Vec::new();

    for log in logs.clone() {
        for (k, v) in log {
            hit_keys.push(k.clone());
            map.push(k, v.as_str());
        }
    }

    c.bench_function(&find_name, |b| {
        b.iter_batched(
            || hit_keys.clone(),
            |keys| {
                for key in keys {
                    black_box(map.find(key));
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_linear_map_v2(c: &mut Criterion, group_name: &str, logs: Vec<Vec<(String, String)>>) {
    let static_logs = leak_dataset_as_static(&logs);

    let insert_name = format!("{group_name}/linear_map_v2/insert");
    let find_name = format!("{group_name}/linear_map_v2/find_hit");

    c.bench_function(&insert_name, |b| {
        b.iter_batched(
            || static_logs.clone(),
            |logs| {
                let mut map = LinearMapV2::new();

                for log in logs {
                    for (k, v) in log {
                        map.push(k, v.as_str());
                    }
                }

                black_box(map);
            },
            BatchSize::SmallInput,
        );
    });

    let mut map = LinearMapV2::new();
    let mut hit_keys = Vec::new();

    for log in static_logs.clone() {
        for (k, v) in log {
            hit_keys.push(k);
            map.push(k, v.as_str());
        }
    }

    c.bench_function(&find_name, |b| {
        b.iter_batched(
            || hit_keys.clone(),
            |keys| {
                for key in keys {
                    black_box(map.find(key));
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_linear_map_v3(c: &mut Criterion, group_name: &str, logs: Vec<Vec<(String, String)>>) {
    let insert_name = format!("{group_name}/linear_map_v3/insert");

    let find_name = format!("{group_name}/linear_map_v3/find_hit");

    // Insertation
    c.bench_function(&insert_name, |b| {
        b.iter_batched(
            || logs.clone(),
            |l| {
                let mut map = LinearMapV3::new();
                for log in l {
                    for (k, v) in log {
                        map.push(k, v);
                    }
                }
                map
            },
            BatchSize::SmallInput,
        );
    });

    let mut map = LinearMapV3::new();
    let mut hit_keys = Vec::new();
    for log in logs {
        for (k, v) in log {
            hit_keys.push(k.clone());
            map.push(k, v);
        }
    }
    //
    // FIND BENCH
    //
    c.bench_function(&find_name, |b| {
        b.iter_batched(
            || hit_keys.clone(),
            |keys| {
                for key in keys {
                    black_box(map.find(&key));
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_hashmap(c: &mut Criterion, group_name: &str, logs: Vec<Vec<(String, String)>>) {
    use map_benchmarks::maps::hm::JSM;

    let insert_name = format!("{group_name}/hashmap/insert");

    let find_name = format!("{group_name}/hashmap/find_hit");

    //
    // INSERT BENCH
    //
    c.bench_function(&insert_name, |b| {
        b.iter_batched(
            || logs.clone(),
            |logs| {
                let mut map = JSM::new();

                for log in logs {
                    for (k, v) in log {
                        map.push(k, v);
                    }
                }

                black_box(map);
            },
            BatchSize::SmallInput,
        );
    });

    //
    // PREBUILD FIND MAP
    //
    let mut map = JSM::new();

    let mut hit_keys = Vec::new();

    for log in logs.clone() {
        for (k, v) in log {
            hit_keys.push(k.clone());

            map.push(k, v);
        }
    }

    //
    // FIND BENCH
    //
    c.bench_function(&find_name, |b| {
        b.iter_batched(
            || hit_keys.clone(),
            |keys| {
                for key in keys {
                    black_box(map.find(&key));
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn benches(c: &mut Criterion) {
    let fw_log = build_fw_log_kv();
    let fw_log_expanded = build_fw_log_expanded_kv();

    bench_linear_map(c, "firewall_log", fw_log.clone());
    bench_linear_map_v2(c, "firewall_log", fw_log.clone());
    bench_linear_map_v3(c, "firewall_log", fw_log.clone());
    bench_hashmap(c, "firewall_log", fw_log);
    
    bench_linear_map(c, "firewall_log_expanded", fw_log_expanded.clone());
    bench_linear_map_v2(c, "firewall_log_expanded", fw_log_expanded.clone());
    bench_linear_map_v3(c, "firewall_log_expanded", fw_log_expanded.clone());
    bench_hashmap(c, "firewall_log_expanded", fw_log_expanded);
    
    
}

criterion_group!(map_bench, benches);
criterion_main!(map_bench);
