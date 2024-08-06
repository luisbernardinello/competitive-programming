import java.util.Scanner;

class Main{

    static long[] fibo(int n) {
        long[] fibSequencia = new long[n];
        int F = 0;     // atual
        int ant = 0;   // anterior

        for (int i = 1; i <= n; i++) {

            if (i == 1) {
                F = 1;
                ant = 0;
            } else {
                F += ant;
                ant = F - ant;
            }

            fibSequencia[i - 1] = F;
        }

        return fibSequencia;
    }

    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);

        int numero = scan.nextInt();
        long[] vetor = Main.fibo(numero);

        for (int i = numero - 1; i >= 0; i--) {
            System.out.print(vetor[i]);
            if (i>0) {
                System.out.print(" ");
            }

        }

        System.out.print("\n");

        scan.close();
    }
}

