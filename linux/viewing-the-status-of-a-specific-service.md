# Viewing the status of a specific service
> 🕰️ logged on 2024.07.30 <br>
> 🔖 tagged: devops, shell, sudo, services, logging, unix

I had to verify that a service on a remote server was still working as intended afer applying a configuration change. Luckily checking this is a breeze, because when you know the name of a service you can see how it is doing with the following command:

```sh
systemctl status <service_name_here>
```

As an example: running `systemctl status lighttpd.service` will yield:

```sh
● lighttpd.service - Lighttpd Daemon
Loaded: loaded (/lib/systemd/system/lighttpd.service; enabled; vendor preset: enabled)
Active: active (running) since Tue 2020-08-04 04:29:19 UTC; 3 weeks 2 days ago
Main PID: 105 (lighttpd)
Tasks: 1 (limit: 115783)
Memory: 56.5M
CGroup: /system.slice/lighttpd.service
        └─105 /usr/sbin/lighttpd -D -f /etc/lighttpd/lighttpd.conf

Aug 04 04:29:19 utls-bash-wiki systemd[1]: Starting Lighttpd Daemon...
Aug 04 04:29:19 utls-bash-wiki systemd[1]: Started Lighttpd Daemon.
```

## Sources
+ [How to view status of a service on Linux using systemctl](https://www.cyberciti.biz/faq/systemd-systemctl-view-status-of-a-service-on-linux/)
