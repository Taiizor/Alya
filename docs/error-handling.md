# Chapter 6: Error Handling

[← Data Structures](data-structures.md) • [Wiki Home](README.md) • [Next: Standard Library →](standard-library.md)

---

## 1. Overview

Alya provides structured exception handling through `try ... catch ... finally ... end`. Unlike traditional C error codes, exceptions in Alya automatically capture runtime errors (such as zero division or out-of-bounds array access) and allow programs to recover gracefully without crashing.

---

## 2. Syntax & Mechanics

### Basic `try ... catch`
```alya
try
    let quotient = 100 / 0
catch err
    say "Intercepted error: " + err
end
```

### The `finally` Clause
Code within the `finally` block is guaranteed to execute whether an error occurred or the `try` block completed successfully:

```alya
try
    say "Executing task..."
catch err
    say "Error occurred: {err}"
finally
    say "Cleanup executed (always runs)!"
end
```

---

## 3. Built-in Runtime Protections

1. **Division & Modulo by Zero**: Alya emits hardware checks before `idiv` / `div` instructions. Dividing or taking the remainder by `0` throws a catchable exception.
2. **Array Bounds Protection**: Indexing past an array's bounds or using negative indices throws an `index out of bounds` error.
3. **Custom Exceptions (`throw`)**: You can explicitly raise exceptions using `throw`:
   ```alya
   throw "Invalid authentication token!"
   ```

---

## 4. Progressive Examples

### Level 1: Pure & Minimal (Division Guard)
```alya
function safe_divide(a, b)
    try
        return a / b
    catch err
        say "Math error caught: {err}"
        return 0
    end
end

say safe_divide(10, 2)   # 5
say safe_divide(10, 0)   # 0
```

---

### Level 2: Practical & Idiomatic (Safe Array Getter with Fallback)
```alya
function array_get_or_default(arr, index, default_val)
    try
        return arr[index]
    catch err
        return default_val
    end
end

let inventory = ["Shield", "Sword", "Potion"]

say array_get_or_default(inventory, 1, "Empty")   # "Sword"
say array_get_or_default(inventory, 10, "Empty")  # "Empty"
say array_get_or_default(inventory, -1, "Empty")  # "Empty"
```

---

### Level 3: Advanced & Real-World (Financial Transaction Gateway with Rollback)
A multi-tier banking transfer pipeline validating account bounds, checking limits, throwing custom errors, and ensuring database locks are released in `finally`:

```alya
function transfer_funds(source_balance, dest_balance, amount)
    let lock_acquired = 1
    say "[LOCK] Database connection locked for transfer."

    try
        # Validate amount
        if amount <= 0
            throw "Transfer amount must be positive!"
        end

        # Check daily limit
        if amount > 1000
            throw "Transfer amount exceeds single-transaction limit of $1000!"
        end

        # Check source funds
        if amount > source_balance
            throw "Insufficient funds in source account!"
        end

        # Process transfer
        let new_source = source_balance - amount
        let new_dest   = dest_balance + amount

        say "[SUCCESS] Transferred ${amount}."
        say "New Source Balance: ${new_source}"
        say "New Destination Balance: ${new_dest}"
        return 1

    catch err
        say "[FAILURE] Transaction Aborted: {err}"
        return 0

    finally
        # Guaranteed cleanup: release database lock
        lock_acquired = 0
        say "[UNLOCK] Database connection safely released."
    end
end

say "--- Test Case 1: Valid Transfer ---"
transfer_funds(500, 100, 150)

say ""
say "--- Test Case 2: Insufficient Balance ---"
transfer_funds(50, 100, 200)

say ""
say "--- Test Case 3: Limit Exceeded ---"
transfer_funds(5000, 100, 1500)
```
