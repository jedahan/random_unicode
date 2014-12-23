What is this I don't even. 30 programs in 30 days?

print all the valid unicode characters:

    target/random_unicode

print all the uniq printable unicode characters:

    target/random_unicode | uniq

print a random character:

    shuf -n 1 <<< $( target/random_unicode | uniq )
