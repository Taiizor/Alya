# Chapter 3: Control Flow

[← Language Basics](basics.md) • [Wiki Home](README.md) • [Next: Functions & Modules →](functions-and-modules.md)

---

## 1. Conditionals (`if ... elif ... else ... end`)

Alya uses clean syntax terminated with the `end` keyword. Under the hood, comparisons in conditional blocks leverage **Branch Fusion**, skipping intermediate boolean materialization for maximum CPU pipelining efficiency.

```alya
let score = 88

if score >= 90
    say "Grade: A"
elif score >= 80
    say "Grade: B"
elif score >= 70
    say "Grade: C"
else
    say "Grade: F"
end
```

---

## 2. Loops & Iteration

### While Loop
Executes as long as the condition evaluates to non-zero:
```alya
let count = 3
while count > 0
    say count
    count -= 1
end
```

### For-in Range Loop
Iterates through an inclusive range of numbers `start..end`:
```alya
for i in 1..5
    say "Iteration #{i}"
end
```

### For-each Collection Loop
Iterates directly over elements of a dynamic array:
```alya
let fruits = ["Apple", "Banana", "Cherry"]
for fruit in fruits
    say "Fruit: {fruit}"
end
```

### Repeat Loop
Loops indefinitely until an explicit `break` condition is met:
```alya
let attempts = 0
repeat
    attempts += 1
    if attempts >= 3
        break
    end
end
```

---

## 3. Loop Control (`break` & `continue`)

* `break`: Immediately exits the nearest enclosing loop.
* `continue`: Skips the remainder of the current iteration and begins the next.

```alya
for i in 1..10
    if i % 2 == 0
        continue    # Skip even numbers
    end
    if i > 7
        break       # Terminate when exceeding 7
    end
    say i           # Prints: 1, 3, 5, 7
end
```

---

## 4. Pattern Matching (`when`)

A concise alternative to chained `if`/`elif` checks:

```alya
let http_status = 404

when http_status
    is 200 then say "OK: Request succeeded"
    is 301 then say "Moved Permanently"
    is 404 then say "Not Found"
    is 500 then say "Internal Server Error"
    else say "Unknown Status Code: {http_status}"
end
```

---

## 5. Progressive Examples

### Level 1: Pure & Minimal (Basic Branch & Count)
```alya
let temperature = 22

if temperature < 15
    say "Cold"
elif temperature <= 25
    say "Comfortable"
else
    say "Warm"
end

let i = 1
while i <= 3
    say "Count: {i}"
    i += 1
end
```

---

### Level 2: Practical & Idiomatic (Prime Number Sieve & Classification)
Finding prime numbers in a range with nested loops and early loop termination:

```alya
let limit = 20
say "Prime numbers up to {limit}:"

for n in 2..limit
    let is_prime = 1
    let d = 2

    while d * d <= n
        if n % d == 0
            is_prime = 0
            break   # Early exit: not prime
        end
        d += 1
    end

    if is_prime == 1
        # Classify the prime category
        when n
            is 2 then say "{n} (The only even prime)"
            is 3 then say "{n} (Smallest odd prime)"
            else say "{n} (Prime)"
        end
    end
end
```

---

### Level 3: Advanced & Real-World (Interactive ATM State Machine)
A robust multi-state financial terminal managing user sessions, menu routing, balance adjustments, and input validation:

```alya
let balance = 500
let is_running = 1
let transactions = 0

say "=== Welcome to Alya Secure ATM ==="

while is_running == 1
    say ""
    say "Current Balance: ${balance}"
    say "1. Deposit Funds"
    say "2. Withdraw Funds"
    say "3. View Session Stats"
    say "4. Exit Terminal"

    let choice = ask "Select an option (1-4): "
    let option = int(choice)

    when option
        is 1 then
            let raw_amount = ask "Enter amount to deposit: $"
            let amount = int(raw_amount)
            if amount > 0
                balance += amount
                transactions += 1
                say "Successfully deposited ${amount}."
            else
                say "Invalid amount: Must be greater than 0."
            end

        is 2 then
            let raw_amount = ask "Enter amount to withdraw: $"
            let amount = int(raw_amount)
            if amount <= 0
                say "Invalid amount: Must be greater than 0."
            elif amount > balance
                say "Declined: Insufficient funds! (Available: ${balance})"
            else
                balance -= amount
                transactions += 1
                say "Dispensing ${amount}... Please collect your cash."
            end

        is 3 then
            say "Session Statistics: {transactions} completed transactions."

        is 4 then
            say "Thank you for banking with Alya. Goodbye!"
            is_running = 0

        else
            say "Unrecognized choice '{choice}'. Please select 1 through 4."
    end
end
```
