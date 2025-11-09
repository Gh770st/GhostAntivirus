#!/usr/bin/env python3

with open('core/src/api/middleware.rs', 'r') as f:
    content = f.read()

# Add imports for rate limiting
if 'use governor::' not in content:
    import_point = content.find('use axum::{')
    if import_point != -1:
        content = content[:import_point] + '''use std::sync::Arc;
use std::net::IpAddr;
use governor::{Quota, RateLimiter};
use governor::state::{InMemoryState, NotKeyed};
use governor::clock::DefaultClock;
use lazy_static::lazy_static;
use std::num::NonZeroU32;

lazy_static! {
    static ref RATE_LIMITER: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>> = {
        // Allow 100 requests per minute
        let quota = Quota::per_minute(NonZeroU32::new(100).unwrap());
        Arc::new(RateLimiter::direct(quota))
    };
}

''' + content[import_point:]

# Replace the rate limiting implementation
old_impl = '''    // TODO: Implement actual rate limiting
    // For now, just pass through
    Ok(next.run(request).await)'''

new_impl = '''    // Check rate limit
    match RATE_LIMITER.check() {
        Ok(_) => {
            // Request allowed
            Ok(next.run(request).await)
        }
        Err(_) => {
            // Rate limit exceeded
            warn!("Rate limit exceeded for request");
            Err(StatusCode::TOO_MANY_REQUESTS)
        }
    }'''

content = content.replace(old_impl, new_impl)

with open('core/src/api/middleware.rs', 'w') as f:
    f.write(content)

print("✅ Implemented rate limiting with governor crate")