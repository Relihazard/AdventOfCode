package main

import (
    "fmt"
    "os"
    "log"
    "bufio"
    "strconv"
)

func main() {
    file, err := os.Open("input")

    if err != nil {
        log.Fatal(err)
        return
    }

    defer file.Close()

    scanner := bufio.NewScanner(file)
    var a []int

    for scanner.Scan() {
        i, err := strconv.Atoi(scanner.Text())
        if err != nil {
            log.Fatal(err)
            return
        }
        if i != 2020 {
            a = append(a, i)
        }
    }

    if err := scanner.Err(); err != nil {
        log.Fatal(err)
        return
    }

    m := make(map[int]bool)
    for i, value := range a {
        curr_sum := 2020 - value

        for _, value2 := range a[i + 1:] {

            if m[curr_sum - value2] {
                fmt.Printf("%d\n", value * value2 * (curr_sum - value2))
                return
            }
            m[value2] = true
        }
    }
}
