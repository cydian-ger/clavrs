# Doc

# Methods
## <b> Read </b>
<b> GET [KEYS] -> [OPTIONAL[VALUES]] </b><br>
returns the value for a key, returns None if key isn't found

<b> EXISTS [KEYS] -> [BOOL] </b><br>
Returns true for every key that exists

<b> HAS [KEYS] [VALUE] -> [BOOL] </b><br>
Returns true for every key that has that value 

---
## <b> Write </b>
<b> PUT LIFETIME [KEYS] [[VALUES]] </b><br>
puts the according value list for every key

<b> DELETE [KEYS] </b><br>
delete keys + values

<b> CLEAR [KEYS]</b><br>
delete values from key, but keep the key

<b> RETRACT [KEYS] [VALUES] </b><br>
Retracts values if present for every key

---
## <b> Write with Return </b>
<b> POP [KEYS] -> [OPTINAL[VALUES]]</b><br>
deletes keys and returns their values, if they exist, else returns None

<b> REDUCE [KEYS] REGEX -> [OPTIONAl[VALUES]] </b><br>
reduces keys with a given regex, all the removed objects are returned.

---
## Life Times
Lifetimes denote how long a variable stay in the database.
- <b><['s] / Static></b> The value is kept in the database
- <b><['d (DATE)] / Date> </b> The data expires by a given date and gets dropped then.
- <b><['u] / User></b> The value lives as long as the user lives.
- <b><['l] / Linked Users></b> The value lives as long as at least 1 user is subscribed to the contract.
- <b><['c] / Connection></b> The value lives as long as the connection lives. A connection is linked to user and thus has the same rights as a user.
- <b><[&LIFETIME] / Reference></b> Reference creates a named reference to a lifetime, that lives at most, as long as the referenced lifetime. <br>
    - Special case <b>['d (DATE)]</b>: Date Lifetime references need to include the date in them -> <b>[&'d (DATE) (NAME)]</b> <br>
![Lifetime](/doc/diagram/Lifetimes.png)

---
## Lifetime Methods
<b> WITH ['LT] / DROP ['LT]</b><br>
Set the default life time for all future put operations. <br>
This default can be overwritten by specifying a lifetime for a put<br>

```txt
WITH ['s]
  PUT ("Key") (("A"))       # Uses Static Lifetime
  PUT ['c] ("Key2") (("B")) # Uses Connection Lifetime

PUT ("Key3") (("C"))        # Uses Default Lifetime
```

With <b>WITH</b> you can also define explicit lifetimes
and with <b>DROP</b> you can drop every key associated under that lifetime.

```
WITH ['c]
  PUT ("Key") (("A"))       # Uses Static Lifetime

DROP ['c] Drop every key related to static
```

Dropping static is not possible. (or at least difficult)

```
CREATE LIFETIME
EXPIRE LIFETIME
```

To create a referenced lifetime it is not enough to evoke it with <b> WITH </b>

```
WITH [&'s (A)]  # Error because A was not created yet
```

You have to manually create it.

```
CREATE [&'s (A)]
WITH [&'s (A)]
  PUT ("KEY1") (("VALUE1"))

DROP [&'s (A)]    # Delete all values associated with this group
EXPIRE [&'s (A)]  # Delete the lifetime reference 
```

So why is it okay to use some lifetimes without creating them.
```
WITH ['s]         # This is okay because life time 's always exists
  PUT ("KEY2") (("VALUE2"))
```
Simply said they are already defined.
Static always exists.
Connection exists as long as you are connected.

---
# IDEA
Andreas R. Schmidt (nicht Peter)

## Ownership
Lifetimes are owned.
Basically:<br>
['s] / ['d] can be <b> borrowed </b> since they aren't directly tied to a connection <br>
['c] / ['u] is <b>owned</b> by the connection.<br>
When you own a lifetime, you can drop it as you are responsible for expiring it (e.g. by ending the connection) [READ WRITE DROP EXPIRE] <br>
When you borrow a lifetime, you can drop it, but can not expire it. [READ, WRITE, DROP] <br>
You can only own something if the access time is equal to or greater than the lifetime. <br>
E.g. if its root (with ['s]) it can own anything since static is the longest possible time.
<br>

## New function
<b> UPDATE [KEY] [VALUES]</b>
Replace the old keys values with new values


<b> MOVE [KEYS] ['LT] </b><br>
```txt
# Moves "Key" to a static life time
MOVE ("Key") ['s]
```

Moves a key from one lifetime to another

Define a sequence as something that runs uninterrupted

think of if there is a lifetime which is dropped when all users relinquish control of it.
['l] for linked.
Then you can do WITH ['l]
and it counts for every person that uses that key.
When all withs are terminated it deletes the key.

# Archive
Maybe allow ownership of lifetimes
with something like <b>TAKE</b>.
Taking would disallow anyone besides the one that took the reference to deref it.
Does this make sense?
Yes, in a sense where
if a lifetime is bound to static it makes sense to be able to deref it from anywhere, but also if a lifetime is a connection bound, it makes sense to only allow the owner of said connection to deref it.<br>
This goes with the Principle: To each to manage their own. Connection data is gonna be exclusively dropped anyway.

You can define a name for any lifetime <br>
WITH ['s := A] <br>
and you can use the <b> DROP (A) </b> without the lifetime itself terminating

Explicit lifetime could also be just a prefix for a lifetime
```
WITH [&'s (A)]
```

```
['e<t>]
['e<'t>]
['e:t]
['e:'t]
['e['t]] 
['e|t|]
['e|'t|]
['e::<t>]
['e::<'t>]
['e::<T>]

['e't = A]
['e<'t> = A]
['e['t] = A]

PUT ['s] ("Key", "Key2") (("K1Value"), ("K2Value"))
```
