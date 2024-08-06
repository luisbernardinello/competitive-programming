
import java.util.Scanner;

class Main {

    static int ladoBem(int h, int e, int a, int x) {
        int calc = h + e + a + x;
        return calc;
    }

    static int ladoMal(int o, int w) {
        int calc = o + w;
        return calc;
    }

    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
        
        int humano = scan.nextInt();
        int elfo = scan.nextInt();
        int anao = scan.nextInt();
        int orc = scan.nextInt();
        int warg = scan.nextInt();
        int aguia = scan.nextInt();

        int timedoBem = Main.ladoBem(humano, elfo, anao, aguia); 
        int timedoMal = Main.ladoMal(orc, warg);

        if (timedoBem >= timedoMal) {
            System.out.print("Middle-earth is safe.");
            System.out.print("\n");
        }
        else {

            System.out.print("Sauron has returned.");
            System.out.print("\n");
        }


        scan.close();
    }
}
