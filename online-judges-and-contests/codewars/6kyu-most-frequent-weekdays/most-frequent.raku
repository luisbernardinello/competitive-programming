use v6;
unit module Solution;

sub mostFrequentDays(Int $year --> Array[Str]) is export {
    my @days = <Monday Tuesday Wednesday Thursday Friday Saturday Sunday>;
    my $first-day = Date.new($year, 1, 1).day-of-week;

    my $days-in-year = Date.new($year, 12, 31).day-of-year;
    my $last-day = Date.new($year, 12, 31).day-of-week;
    my @result;

    if $days-in-year == 365 {
        @result.push(@days[$first-day - 1]);
    } else {
        @result.push(@days[$first-day - 1], @days[$last-day - 1]);
    }

    @result = @result.unique.sort({ @days.index($_) });

    return Array[Str].new(@result);
}