# Use the official Fluentd image as base
FROM fluent/fluentd:v1.16-debian-1

# Install required plugins
USER root
RUN gem install fluent-plugin-s3 \
    && gem install fluent-plugin-concat \
    && gem install fluent-plugin-rewrite-tag-filter

# Install Go
RUN apt-get update && apt-get install -y golang \
    && rm -rf /var/lib/apt/lists/*

# Install columnify and copy the binary to a location in PATH
RUN go install github.com/reproio/columnify/cmd/columnify@latest \
    && cp /root/go/bin/columnify /usr/local/bin/

# Copy configuration files
COPY ./fluentd/conf /fluentd/etc

# Create directories for logs
RUN mkdir -p /fluentd/log /app/logs

# Set permissions
RUN chmod 644 /app/logs \
    && chown -R fluent:fluent /app/logs \
    && chown -R fluent:fluent /fluentd/log

# Switch back to fluent user
USER fluent

# Expose ports
EXPOSE 24224 24224/udp

# Start Fluentd
CMD ["fluentd", "-c", "/fluentd/etc/fluent.conf", "-p", "/fluentd/plugins"]
