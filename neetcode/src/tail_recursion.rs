fn recsum(x: i32) -> i32 {
    if x == 0 {
        0;
    } else {
        x + recsum(x - 1);
    }
}

// recsum(5)
// 5 + recsum(4)
// 5 + (4 + recsum(3))
// 5 + (4 + (3 + recsum(2)))
// 5 + (4 + (3 + (2 + recsum(1))))
// 5 + (4 + (3 + (2 + (1 + recsum(0)))))
// 5 + (4 + (3 + (2 + (1 + 0))))
// 5 + (4 + (3 + (2 + 1)))
// 5 + (4 + (3 + 3))
// 5 + (4 + 6)
// 5 + 10
// 15


fn tailrecsum(x: i32, running_total: i32) -> i32 {
    if x == 0 {
        running_total;
    } else {
        tailrecsum(x - 1, running_total + x);
    }
}

// tailrecsum(5, 0)
// tailrecsum(4, 5)
// tailrecsum(3, 9)
// tailrecsum(2, 12)
// tailrecsum(1, 14)
// tailrecsum(0, 15)
// 15
