use des::read_2d_array;
use std::env;

struct Bivariate {
    u: f64,
    v: f64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let arr = read_2d_array(filename);

    let mut index: i64 = 0;
    let mut sum = Bivariate { u: 0.0, v: 0.0 };
    let mut mean = Bivariate { u: 0.0, v: 0.0 };
    let mut min = Bivariate {
        u: f64::INFINITY,
        v: f64::INFINITY,
    };
    let mut max = Bivariate {
        u: f64::NEG_INFINITY,
        v: f64::NEG_INFINITY,
    };
    let mut diff = Bivariate { u: 0.0, v: 0.0 };
    let mut cosum: f64 = 0.0;
    let correlation: f64;
    let mut temp: f64;
    let theta: f64;
    let pi: f64 = 4.0 * (1.0 as f64).atan();

    for a in arr {
        index += 1;
        temp = ((index as f64) - 1.0) / (index as f64);
        diff.u = a[0] - mean.u;
        diff.v = a[1] - mean.v;
        sum.u += diff.u * diff.u * temp;
        sum.v += diff.v * diff.v * temp;
        cosum += diff.u * diff.v * temp;
        mean.u += diff.u / (index as f64);
        mean.v += diff.v / (index as f64);
        if a[0] > max.u {
            max.u = a[0];
        } else if a[0] < min.u {
            min.u = a[0];
        }
        if a[1] > max.v {
            max.v = a[1];
        } else if a[1] < min.v {
            min.v = a[1];
        }
    }

    if index > 0 {
        let stdev = Bivariate {
            u: (sum.u / (index as f64)).sqrt(),
            v: (sum.v / (index as f64)).sqrt(),
        };
        let covariance = cosum / (index as f64);
        if stdev.u * stdev.v > 0.0 {
            correlation = covariance / (stdev.u * stdev.v);
        } else {
            correlation = 0.0;
        }
        sum.u = (stdev.u * stdev.u) - (stdev.v * stdev.v);
        sum.v = 2.0 * covariance;
        theta = 0.5 * (sum.v).atan2(sum.u);
        println!("\nfor a bivariate sample of size {index}");
        println!("\nmean.u ...... = {}", mean.u);
        println!("stdev.u ..... = {}", stdev.u);
        println!("min.u ....... = {}", min.u);
        println!("max.u ....... = {}", max.u);
        println!("\nmean.v ...... = {}", mean.v);
        println!("stdev.v ..... = {}", stdev.v);
        println!("min.v ....... = {}", min.v);
        println!("max.v ....... = {}", max.v);
        println!("\ncorrelation ...... = {}", correlation);
        println!("theta (radians) .. = {}", theta);
        println!("theta (degrees) .. = {}", 180.0 * theta / pi);
    }
}
