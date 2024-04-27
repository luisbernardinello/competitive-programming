sub MAIN() {
    my $value-str = prompt("entre o valor: ");
    my $value = $value-str.trim.parse(f64) // die "valor invalido";

    my $hundred = 0;
    my $fifty = 0;
    my $twenty = 0;
    my $ten = 0;
    my $five = 0;
    my $two = 0;
    my $one = 0;
    my $fiftycents = 0;
    my $twentyfivecents = 0;
    my $tencents = 0;
    my $fivecents = 0;
    my $cents = 0;

    my $value = $value * 100; # converte para centavos

    if ($value / 100) >= 1 {
        $hundred = ($value / 100).Int;
        $value -= $hundred * 100;
    }

    if ($value / 50) >= 1 {
        $fifty = ($value / 50).Int;
        $value -= $fifty * 50;
    }

    if ($value / 20) >= 1 {
        $twenty = ($value / 20).Int;
        $value -= $twenty * 20;
    }

    if ($value / 10) >= 1 {
        $ten = ($value / 10).Int;
        $value -= $ten * 10;
    }

    if ($value / 5) >= 1 {
        $five = ($value / 5).Int;
        $value -= $five * 5;
    }

    if ($value / 2) >= 1 {
        $two = ($value / 2).Int;
        $value -= $two * 2;
    }

    if ($value / 1) >= 1 {
        $one = ($value / 1).Int;
        $value -= $one * 1;
    }

    if ($value / 0.50) >= 1 {
        $fiftycents = ($value / 0.50).Int;
        $value -= $fiftycents * 0.50;
    }

    if ($value / 0.25) >= 1 {
        $twentyfivecents = ($value / 0.25).Int;
        $value -= $twentyfivecents * 0.25;
    }

    if ($value / 0.10) >= 1 {
        $tencents = ($value / 0.10).Int;
        $value -= $tencents * 0.10;
    }

    if ($value / 0.05) >= 1 {
        $fivecents = ($value / 0.05).Int;
        $value -= $fivecents * 0.05;
    }

    if ($value / 0.01) >= 0.998 {
        $cents = ($value / 0.01).Int;
    }

    say "NOTAS:";
    say "{$hundred} nota(s) de R$ 100.00";
    say "{$fifty} nota(s) de R$ 50.00";
    say "{$twenty} nota(s) de R$ 20.00";
    say "{$ten} nota(s) de R$ 10.00";
    say "{$five} nota(s) de R$ 5.00";
    say "{$two} nota(s) de R$ 2.00";
    say "MOEDAS:";
    say "{$one} moeda(s) de R$ 1.00";
    say "{$fiftycents} moeda(s) de R$ 0.50";
    say "{$twentyfivecents} moeda(s) de R$ 0.25";
    say "{$tencents} moeda(s) de R$ 0.10";
    say "{$fivecents} moeda(s) de R$ 0.05";
    say "{$cents} moeda(s) de R$ 0.01";
}
