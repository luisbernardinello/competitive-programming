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

    my $value-cents = $value * 100; # converte para centavos

    if ($value-cents / 100) >= 1 {
        $hundred = ($value-cents / 100).Int;
        $value-cents -= $hundred * 100;
    }

    if ($value-cents / 50) >= 1 {
        $fifty = ($value-cents / 50).Int;
        $value-cents -= $fifty * 50;
    }

    if ($value-cents / 20) >= 1 {
        $twenty = ($value-cents / 20).Int;
        $value-cents -= $twenty * 20;
    }

    if ($value-cents / 10) >= 1 {
        $ten = ($value-cents / 10).Int;
        $value-cents -= $ten * 10;
    }

    if ($value-cents / 5) >= 1 {
        $five = ($value-cents / 5).Int;
        $value-cents -= $five * 5;
    }

    if ($value-cents / 2) >= 1 {
        $two = ($value-cents / 2).Int;
        $value-cents -= $two * 2;
    }

    if ($value-cents / 1) >= 1 {
        $one = ($value-cents / 1).Int;
        $value-cents -= $one * 1;
    }

    if ($value-cents / 50) >= 1 {
        $fiftycents = ($value-cents / 50).Int;
        $value-cents -= $fiftycents * 50;
    }

    if ($value-cents / 25) >= 1 {
        $twentyfivecents = ($value-cents / 25).Int;
        $value-cents -= $twentyfivecents * 25;
    }

    if ($value-cents / 10) >= 1 {
        $tencents = ($value-cents / 10).Int;
        $value-cents -= $tencents * 10;
    }

    if ($value-cents / 5) >= 1 {
        $fivecents = ($value-cents / 5).Int;
        $value-cents -= $fivecents * 5;
    }

    if ($value-cents / 1) >= 0.998 {
        $cents = ($value-cents / 1).Int;
    }

    say "NOTAS:";
    say "{$hundred} nota(s) de R\$ 100.00";
    say "{$fifty} nota(s) de R\$ 50.00";
    say "{$twenty} nota(s) de R\$ 20.00";
    say "{$ten} nota(s) de R\$ 10.00";
    say "{$five} nota(s) de R\$ 5.00";
    say "{$two} nota(s) de R\$ 2.00";
    say "MOEDAS:";
    say "{$one} moeda(s) de R\$ 1.00";
    say "{$fiftycents} moeda(s) de R\$ 0.50";
    say "{$twentyfivecents} moeda(s) de R\$ 0.25";
    say "{$tencents} moeda(s) de R\$ 0.10";
    say "{$fivecents} moeda(s) de R\$ 0.05";
    say "{$cents} moeda(s) de R\$ 0.01";
}
