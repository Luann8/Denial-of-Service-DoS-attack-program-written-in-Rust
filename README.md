## Multi-threaded TCP Client Instructions

### Overview:
This Rust program creates multiple threads to concurrently send TCP requests to a specified IP address and port. It counts the number of successful connections and prints the count.

### Instructions:
1. Replace `"TARGET_IP_ADDRESS"` and `"TARGET_PORT"` with the IP address and port of the target server you want to connect to.

2. Ensure you have Rust installed on your system. If not, you can download and install it from [Rust's official website](https://www.rust-lang.org/tools/install).

3. Copy the provided code into a Rust source file (e.g., `main.rs`).

4. Open a terminal or command prompt and navigate to the directory containing your Rust source file.

5. Compile and run the program using the following command:
    ```bash
    cargo run
    ```

6. Observe the output, which will display the number of successful connections made by the client.

7. You can adjust the number of threads (`num_threads`) in the code according to your requirements.

8. Press `Ctrl + C` to stop the program.

### Notes:
- This program is for educational purposes and may need modifications for use in production environments.
- Make sure you have permission to connect to the target IP address and port.
