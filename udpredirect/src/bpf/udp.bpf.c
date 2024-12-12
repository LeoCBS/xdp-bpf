#include <linux/bpf.h>

#include <bpf/bpf_endian.h>
#include <bpf/bpf_helpers.h>

#include <arpa/inet.h>
#include <linux/if_ether.h>
#include <linux/in.h>
#include <linux/ip.h>
#include <linux/udp.h>

#define DEFAULT_QUEUE_IDS_SIZE 64

struct xdp_meta {
  __u64 rx_timestamp;
};

struct {
  __uint(type, BPF_MAP_TYPE_XSKMAP);
  __uint(key_size, sizeof(int));
  __uint(value_size, sizeof(int));
  __uint(max_entries, DEFAULT_QUEUE_IDS_SIZE);
} xsks_map SEC(".maps");

SEC("xdp")
int udp_capture(struct xdp_md *ctx) {
  void *data, *data_meta, *data_end;
  // this struct represent raw ethernet packet
  struct ethhdr *eth = NULL;
  // this  struct represent a UDP layer
  struct udphdr *udp = NULL;
  // this  struct represent a IP layer
  struct iphdr *iph = NULL;
  struct xdp_meta *meta;
  int err;

  // bpf_printk("data %s", eth->h_proto);

  // The data field points to the start of the packet, the data_end field points
  // to the end.
  // To satisfy the compiler type checking we need to cast the fields to
  // pointers
  data = (void *)(long)ctx->data;
  data_end = (void *)(long)ctx->data_end;

  // represent size of struct ethhdr that is  6(ETH_ALEN)+6+2 = 14 bytes
  __u32 nh_off;
  nh_off = sizeof(struct ethhdr);
  // check if eth packet isn't corrupted, this check says that data start plus
  // ethhfr struct size isn't out of range
  if (data + sizeof(struct ethhdr) < data_end) {
    bpf_printk("data end - data %d", data_end - data);
    bpf_printk("data %d", data);
    bpf_printk("data + ethhdr size %d", data + nh_off);
    // define eth pointer to equal data pointer, because eth still null and
    // The data starts with an Ethernet header, so we assign the data to ethhdr
    // like this:
    eth = data;
    if (eth->h_proto == bpf_htons(ETH_P_IP)) {
      // iph pointer start size of struct eth plus 1
      iph = (void *)(eth + 1);
      if ((void *)(iph + 1) < data_end && iph->protocol == IPPROTO_UDP)
        // udp pointer start size of iph struct plus 1
        udp = (void *)(iph + 1);
    }
    if (udp && (void *)(udp + 1) > data_end)
      udp = NULL;
  }

  if (!udp)
    return XDP_PASS;

  if (udp->dest != htons(1025)) {
    bpf_printk("by passing udp dest port %d", udp->dest);
    return XDP_DROP;
  }
  bpf_printk("udp dest port %d", htons(udp->dest));

  err = bpf_xdp_adjust_meta(ctx, -(int)sizeof(struct xdp_meta));
  if (err)
    return XDP_PASS;

  data = (void *)(long)ctx->data;
  data_meta = (void *)(long)ctx->data_meta;
  meta = data_meta;

  if ((void *)(meta + 1) > data)
    return XDP_PASS;

  bpf_printk("rx_queue_indexxxxx %d", ctx->rx_queue_index);
  return bpf_redirect_map(&xsks_map, ctx->rx_queue_index, XDP_PASS);
}

char _license[] SEC("license") = "GPL";
