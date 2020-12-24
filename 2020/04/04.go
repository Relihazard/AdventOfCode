package main

import (
    "fmt"
    "os"
    "bufio"
    "log"
    "strings"
    "sort"
)

func main() {
    file, err := os.Open("input")

    if err != nil {
        log.Fatal(err)
        return
    }

    defer file.Close()

    scanner := bufio.NewScanner(file)
    var counter int
    var a []string

    for scanner.Scan() {
        line := scanner.Text()
        if len(line) == 0 {
            len_a := len(a)
            if len_a == 8 || (len_a == 7 && sort.SearchStrings(a, "cid") == 0) {
                counter++
            }
            a = nil
            continue
        }

        splits := strings.Split(line, " ")

        for _, value := range splits {
            field := strings.Split(value, ":")[0]
            a = append(a, field)
        }
    }

    if err := scanner.Err(); err != nil {
        log.Fatal(err)
        return
    }

    fmt.Printf("%d\n", counter)
}
