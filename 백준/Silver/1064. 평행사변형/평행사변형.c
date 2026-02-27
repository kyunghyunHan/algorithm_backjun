#include <stdio.h>
#include <math.h>

double dist(double x1, double y1, double x2, double y2) {
    double dx = x1 - x2;
    double dy = y1 - y2;
    return sqrt(dx * dx + dy * dy);
}

int main() {
    double xA, yA, xB, yB, xC, yC;
    scanf("%lf %lf %lf %lf %lf %lf", 
          &xA, &yA, &xB, &yB, &xC, &yC);

    double cross = (xB - xA) * (yC - yA) 
                 - (yB - yA) * (xC - xA);

    if (fabs(cross) < 1e-12) {
        printf("-1.0\n");
        return 0;
    }

    double AB = dist(xA, yA, xB, yB);
    double AC = dist(xA, yA, xC, yC);
    double BC = dist(xB, yB, xC, yC);

    double P1 = 2 * (AB + AC);
    double P2 = 2 * (AB + BC);
    double P3 = 2 * (AC + BC);

    double maxP = fmax(P1, fmax(P2, P3));
    double minP = fmin(P1, fmin(P2, P3));

    printf("%.15f\n", maxP - minP);

    return 0;
}