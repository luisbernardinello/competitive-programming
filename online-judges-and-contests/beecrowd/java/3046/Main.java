
import java.util.Scanner;

class Main {

    static int calc(int n) {
        int formula = ((n + 1) * (n + 2)) / 2;
        return formula;
    }

    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);

        int numero = scan.nextInt();

        int resultado = Main.calc(numero);

        System.out.print(resultado);

        System.out.print("\n");

        scan.close();
    }
}

