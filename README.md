# toydig
- dig clone

# サンプルデータの作成
digコマンドを使用してクエリパケットの内容を取得します。
digコマンドは次のように利用する
```bash
dig [options] @[DNSサーバーIPアドレス] [ドメイン名] [クエリタイプ]
```

(例)
google public dns(8.8.8.8)経由でyahoo.co.jpを探す
```bash
dig @8.8.8.8 yahoo.co.jp

; <<>> DiG 9.20.18 <<>> @8.8.8.8 yahoo.co.jp
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 60157
;; flags: qr rd ra; QUERY: 1, ANSWER: 18, AUTHORITY: 0, ADDITIONAL: 1

;; OPT PSEUDOSECTION:
; EDNS: version: 0, flags:; udp: 512
;; QUESTION SECTION:
;yahoo.co.jp.			IN	A

;; ANSWER SECTION:
yahoo.co.jp.		175	IN	A	183.79.249.252
yahoo.co.jp.		175	IN	A	182.22.25.124
yahoo.co.jp.		175	IN	A	182.22.31.124
yahoo.co.jp.		175	IN	A	182.22.25.252
yahoo.co.jp.		175	IN	A	183.79.249.124
yahoo.co.jp.		175	IN	A	182.22.16.251
yahoo.co.jp.		175	IN	A	183.79.219.124
yahoo.co.jp.		175	IN	A	183.79.250.251
yahoo.co.jp.		175	IN	A	124.83.185.124
yahoo.co.jp.		175	IN	A	182.22.24.124
yahoo.co.jp.		175	IN	A	124.83.185.252
yahoo.co.jp.		175	IN	A	182.22.16.123
yahoo.co.jp.		175	IN	A	124.83.184.124
yahoo.co.jp.		175	IN	A	124.83.184.252
yahoo.co.jp.		175	IN	A	182.22.31.252
yahoo.co.jp.		175	IN	A	183.79.219.252
yahoo.co.jp.		175	IN	A	182.22.28.252
yahoo.co.jp.		175	IN	A	182.22.24.252

;; Query time: 5 msec
;; SERVER: 8.8.8.8#53(8.8.8.8) (UDP)
;; WHEN: Sun Mar 01 20:47:20 JST 2026
;; MSG SIZE  rcvd: 328
```

`@8.8.8.8`をlocalhostに指定すればローカルDNSサーバーに向けてクエリを発行出来る
そのときのdigコマンドが送信するパケットを取得する

手軽のためnetcatでdigのDNSクエリを待ち受けよう。DNSは通常UDPで通信される。
ポート番号1053に特に理由はない。
```bash
nc -u -l 1053 > query_packet.txt
```

ターミナルをもう一つ開いてdigコマンドでlocalhostに向けてクエリを飛ばす

```bash
dig +retry=0 +noedns -p 1053 @127.0.0.1 google.com
```

これでquery_packet.txtが得られる

query_packet.txtを利用してこれをnetcatでgoogle public dns(8.8.8.8)に飛ばしてみる

```bash
nc -u 8.8.8.8 53 < query_packet.txt > response_packet.txt
```

それぞれの中身を見てみよう

```bash
$ hexdump -C ./data/query_packet.txt
00000000  aa 46 01 20 00 01 00 00  00 00 00 00 06 67 6f 6f  |.F. .........goo|
00000010  67 6c 65 03 63 6f 6d 00  00 01 00 01              |gle.com.....|
0000001c

$ hexdump -C ./data/response_packet.txt
00000000  aa 46 81 80 00 01 00 01  00 00 00 00 06 67 6f 6f  |.F...........goo|
00000010  67 6c 65 03 63 6f 6d 00  00 01 00 01 c0 0c 00 01  |gle.com.........|
00000020  00 01 00 00 01 2c 00 04  8e fa c2 0e              |.....,......|
0000002c
```


# References
- [dnsguide](https://github.com/EmilHernvall/dnsguide)
- [dog](https://github.com/ogham/dog)
- [RFC 1035 - DN](https://datatracker.ietf.org/doc/html/rfc1035)
- [RFC 6819 - EDNS](https://datatracker.ietf.org/doc/html/rfc6891)
