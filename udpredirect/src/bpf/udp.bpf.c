#include <linux/bpf.h>

#include <bpf/bpf_endian.h>
#include <bpf/bpf_helpers.h>

#include <linux/if_ether.h>
#include <linux/in.h>
#include <linux/ip.h>
#include <linux/udp.h>

#define DEFAULT_QUEUE_IDS 64

struct xdp_meta {
  __u64 rx_timestamp;
};

struct {
  __uint(type, BPF_MAP_TYPE_XSKMAP);
  __uint(key_size, sizeof(int));
  __uint(value_size, sizeof(int));
  __uint(max_entries, DEFAULT_QUEUE_IDS);
} xsks_map SEC(".maps");

SEC("xdp")
int udp_capture(struct xdp_md *ctx) {
  void *data, *data_meta, *data_end;
  struct ethhdr *eth = NULL;
  struct udphdr *udp = NULL;
  struct iphdr *iph = NULL;
  struct xdp_meta *meta;
  int err;

  data = (void *)(long)ctx->data;
  data_end = (void *)(long)ctx->data_end;

  if (data + sizeof(struct ethhdr) < data_end) {
    eth = data;
    if (eth->h_proto == bpf_htons(ETH_P_IP)) {
      iph = (void *)(eth + 1);
      if ((void *)(iph + 1) < data_end && iph->protocol == IPPROTO_UDP)
        udp = (void *)(iph + 1);
    }
    if (udp && (void *)(udp + 1) > data_end)
      udp = NULL;
  }

  if (!udp)
    return XDP_PASS;

  err = bpf_xdp_adjust_meta(ctx, -(int)sizeof(struct xdp_meta));
  if (err)
    return XDP_PASS;

  data = (void *)(long)ctx->data;
  data_meta = (void *)(long)ctx->data_meta;
  meta = data_meta;

  if ((void *)(meta + 1) > data)
    return XDP_PASS;

  meta->rx_timestamp = bpf_ktime_get_tai_ns();

  return bpf_redirect_map(&xsks_map, ctx->rx_queue_index, XDP_PASS);
}

char _license[] SEC("license") = "GPL";
