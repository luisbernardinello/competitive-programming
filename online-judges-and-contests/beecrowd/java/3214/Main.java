
import java.util.Scanner;

class Main {

    static int calcRefrisTomados(int garrafas, int valor) {
        int total = 0;
        
        while (garrafas >= valor) {
            int novosRefris = garrafas / valor;
            total += novosRefris;
            garrafas = novosRefris + (garrafas % valor);
        }

        return total;
    }

    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
       
        int garrafasVaziasInicial = scan.nextInt();
        int garrafasVaziasEncontradas = scan.nextInt();
        int valorTroca = scan.nextInt();

        int calc = Main.calcRefrisTomados(garrafasVaziasInicial + garrafasVaziasEncontradas, valorTroca);
        System.out.println(calc);

        scan.close();
    }
}

