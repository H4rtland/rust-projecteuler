import std.stdio;

void main() {
    int diagonal_sum = 1;
    int current_value = 1;
    int side_length = 1;
    int digits_until_diagonal = side_length+1;
    int sides_until_size_up = 4;
    
    while(side_length < 1001) {
        current_value += 1;
        digits_until_diagonal -= 1;
        if (digits_until_diagonal == 0) {
            diagonal_sum += current_value;
            sides_until_size_up -= 1;
            if (sides_until_size_up == 0) {
                sides_until_size_up = 4;
                side_length += 2;
            }
            digits_until_diagonal = side_length+1;
        }
    }
    writeln(diagonal_sum);
}