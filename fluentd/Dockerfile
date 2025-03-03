# Use the official Fluentd image as base
FROM fluent/fluentd:v1.16-debian-1

# Install required plugins
USER root
RUN gem install fluent-plugin-s3 \
    && gem install fluent-plugin-concat \
    && gem install fluent-plugin-rewrite-tag-filter

# Copy configuration files
COPY ./fluentd/conf/fluent.conf /fluentd/etc/

# Create directories for logs
RUN mkdir -p /fluentd/log

# Set permissions
RUN chown -R fluent:fluent /fluentd/log

# Switch back to fluent user
USER fluent

# Expose ports
EXPOSE 24224 24224/udp

# Start Fluentd
CMD ["fluentd", "-c", "/fluentd/etc/fluent.conf", "-p", "/fluentd/plugins"]
