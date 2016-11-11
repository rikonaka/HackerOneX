# This is a search tool for search Vulnerability

before you make it with gcc, you must confirm you have installed the `libcurl`

if not, try this:

```
sudo apt-get install libcurl4-openssl-dev

```

## You can make it as

```
gcc *.c -o svf -lm -l curl
```

## And run it like this

```
./svf discuz
```

and the result it like below:

```
Get the API data retrieved
NOW LIST THE ALL RESULT:
NUML NUMC | DATE       | DETAIL
>[1] 6792 | 2010-01-29 | Discuz! 6.0.0 cross site scripting
>[2] 7570 | 2009-09-17 | Discuz! Plugin Crazy Star <= 2.0 (fmid) SQL Injection Vulnerability
>[3] 7619 | 2009-09-15 | Discuz! JiangHu plugin versions 1.1 and below remote SQL injection
>[4] 7779 | 2009-08-25 | Discuz 6.0 (2fly_gift.php) Sql Injection Vulnerability
>[5] 7878 | 2009-08-19 | Discuz! Remote Reset User Password Exploit
>[6] 7879 | 2009-08-19 | Discuz! 6.x/7.x Remote Code Execution Exploit
>[7] 10534 | 2008-08-07 | Comsenz Discuz! 6.0.1 Sql injection
>[8] 6792 | 2010-01-29 | Discuz! 6.0.0 cross site scripting
>[9] 7570 | 2009-09-17 | Discuz! Plugin Crazy Star <= 2.0 (fmid) SQL Injection Vulnerability
>[10] 7619 | 2009-09-15 | Discuz! JiangHu plugin versions 1.1 and below remote SQL injection
>[11] 7779 | 2009-08-25 | Discuz 6.0 (2fly_gift.php) Sql Injection Vulnerability
>[12] 7878 | 2009-08-19 | Discuz! Remote Reset User Password Exploit
>[13] 7879 | 2009-08-19 | Discuz! 6.x/7.x Remote Code Execution Exploit
>[14] 10534 | 2008-08-07 | Comsenz Discuz! 6.0.1 Sql injection
END
```

## NOTE:
- The `NUML` is mean the serial number in local machine
- The `NUMC` is mean the serial number of CVE
