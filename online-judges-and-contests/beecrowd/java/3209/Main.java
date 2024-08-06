
import java.util.Scanner;

class Main {

    static int calcTomadasFiltros(int[] vet) {
        int totalTomadas = 0;
        for (int i = 0; i < vet.length; i++) {
            totalTomadas += vet[i];
        }
        return totalTomadas - (vet.length - 1);
    }

    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
       
        int cases = scan.nextInt();
        
        while (cases-- > 0) {
            int filtros = scan.nextInt();
            int tomadas[] = new int[filtros];
            for (int i = 0; i < filtros; i++) {
                tomadas[i] = scan.nextInt();
            }
         
            int calc = Main.calcTomadasFiltros(tomadas);
            System.out.println(calc);
        }

        scan.close();
    }
}

