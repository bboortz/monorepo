+++
title = "How to flush DNS Cache in Firefox"
date = "2021-09-15T20:34:30+02:00"
author = ""
authorTwitter = "" #do not include @
cover = ""
tags = []
keywords = ["", ""]
description = ""
showFullContent = true
+++

Today I have moved the domain record **benn.tech** from based an ip address to a resolution using IPFS based on fleek. For DNS is am using [Namecheap](https://namecheap.com/). This is how the setup now looks like:

| Type  | Host         | Value                           |
------- | ------------ | ------------------------------- |
| CNAME | _dnslink.www | _dnslink.bennitech.on.fleek.co. |
| CNAME | _dnslink     | _dnslink.bennitech.on.fleek.co. |
| CNAME | @            | 4cbeeb4387fc7eacf4a4.b-cdn.net. |
| CNAME | www          | 4cbeeb4387fc7eacf4a4.b-cdn.net. |

I have verified the DNS setup using `host -a -t any benni.tech 8.8.8.8`. The response was good.
Also [DNS Checker](https://dnschecker.org/#CNAME/benni.tech) was showng the correct response.

But a `host -a -t any benni.tech` which is served by my local dns server based on [Pihole](https://pi-hole.net) was still showing the wrong ip. A `pihole restartdns` helped here before the TTL has expired.
So that curl and other tools were showing the right responses.

Firefox was still showing the old page. So that I have cleared the DNS cache in firefox using this [entry](https://stackoverflow.com/questions/59932525/how-can-i-force-firefox-to-reset-its-dns-cache-on-demand) on Stackoverflow. I have just opened `about:networking#dns` and have pressed the button `Clear DNS Cache`.
This solved the problem.

This blog is now available via:
* https://ipfs.io/ipns/benni.tech/
* https://ipfs.io/ipfs/QmdxaxczCgnNgDA4wy6gjAJoKqXFKJiYhquKYqUHMk4ysu/
* https://bennitech.on.fleek.co/
* https://ipfs.fleek.co/ipfs/QmdxaxczCgnNgDA4wy6gjAJoKqXFKJiYhquKYqUHMk4ysu/
* https://benni.tech/




