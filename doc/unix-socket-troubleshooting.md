# Unix Socket Troubleshooting

1. Change default socket in `spnav.c`:
    ```c
    #define SPNAV_SOCK_PATH "/var/run/spnav-mock.sock"
    ```

2. Run socat to inspect traffic:
    ```bash
    sudo socat -t100 -x -v UNIX-LISTEN:/var/run/spnav-mock.sock,mode=777,reuseaddr,fork UNIX-CONNECT:/var/run/spnav.sock
    ```
