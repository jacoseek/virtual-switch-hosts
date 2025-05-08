export enum HostType {
  Local,
  Remote,
}

export interface HostItem {
  id: string;
  name: string;
  type: HostType;
  remoteUrl?: string;
  hosts?: string;
  enabled: boolean;
}

export interface VpnConfig {
  address: string;
  dnsServer: string;
  allowedApplications: string[];
  disallowedApplications: string[];
  mtu: number;
}

export const defaultVpnConfig = {
  address: '192.0.2.111/32',
  dnsServer: '8.8.8.8',
  allowedApplications: [],
  disallowedApplications: [],
  mtu: 1300,
};

export const useHostsStore = defineStore('hosts', () => {
  const list = ref<HostItem[]>([]);
  const activeHosts = ref('');
  const isProxying = ref(false);
  const vpnConfig = ref<VpnConfig>({
    ...defaultVpnConfig,
  });

  watch(
    list,
    async (val) => {
      window.$storage.set('hostsList', val);
      activeHosts.value = await getActiveHosts(list.value);
    },
    { deep: true },
  );

  watch(activeHosts, async (val) => {
    commands.setHosts(val);
  });

  watch(
    vpnConfig,
    async (val) => {
      window.$storage.set('vpnConfig', val);
    },
    { deep: true },
  );

  const init = async () => {
    list.value = (await window.$storage.get('hostsList')) || [];
    vpnConfig.value = (await window.$storage.get('vpnConfig')) || vpnConfig.value;
  };

  return {
    list,
    activeHosts,
    isProxying,
    vpnConfig,
    init,
  };
});

const getActiveHosts = async (list: HostItem[]) => {
  const activeList = list.filter((item) => item.enabled);
  const promises = activeList.map((item) => {
    if (item.type === HostType.Remote && item.remoteUrl) {
      return fetch(item.remoteUrl)
        .then((response) => response.text())
        .then((data) => {
          return {
            name: item.name,
            hosts: data,
          };
        })
        .catch((error) => {
          console.error('Error:', error);
          return item;
        });
    }
    return {
      name: item.name,
      hosts: item.hosts,
    };
  });
  const res = await Promise.all(promises);
  return res.map((item) => `# ${item.name} \n${item.hosts} \n`).join('\n');
};
