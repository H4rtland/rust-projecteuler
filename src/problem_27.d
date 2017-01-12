import std.stdio;
import std.conv;
import std.datetime : StopWatch;

bool is_prime(int n) {
    if (n <= 1) {
        return false;
    }
    foreach(i; 2..n/2) {
        if (n % i == 0) {
            return false;
        }
    }
    return true;
}

int sequence_length(int a, int b) {
    int n = 0;
    while (true) {
        if (!is_prime((n*n) + (a*n) + b)) {
            break;
        }
        n += 1;
    }
    return n;
}

void test() {
    int max_len = 0;
    int max_a = 0;
    int max_b = 0;
    foreach(a; -999..1000) {
        foreach(b; -1000..1001) {
            int seq_len = sequence_length(a, b);
            if (seq_len > max_len) {
                max_len = seq_len;
                max_a = a;
                max_b = b;
            }
        }
    }
    writeln("Results: a=", max_a, ", b=", max_b, ", product=", max_a*max_b);
}

void main(string[] args){
    StopWatch timer;
    timer.start();
    test();
    timer.stop();
    writeln("Took ", cast(float)timer.peek().msecs/1000.0, " seconds");
}