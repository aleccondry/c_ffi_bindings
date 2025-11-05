#include <math.h>

typedef struct {
    int x;
    int y;
} Point;

float distance(const Point* p1, const Point* p2) {
    int dx = p2->x - p1->x;
    int dy = p2->y - p1->y;
    return sqrtf(dx * dx + dy * dy);
}


