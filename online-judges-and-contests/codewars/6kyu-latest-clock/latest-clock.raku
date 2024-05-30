use v6;
unit module Solution;

sub latest-clock(Int $a, Int $b, Int $c, Int $d) is export {
    my @digits = ($a, $b, $c, $d);
    my $latest-time = -1;
    my $result = "00:00";

    for @digits.permutations -> @perm {
        my $hours = @perm[0] ~ @perm[1];
        my $minutes = @perm[2] ~ @perm[3];

        if $hours.Int < 24 && $minutes.Int < 60 {
            my $current-time = $hours.Int * 60 + $minutes.Int;
            if $current-time > $latest-time {
                $latest-time = $current-time;
                $result = $hours ~ ':' ~ $minutes;
            }
        }
    }
    return $result;
}