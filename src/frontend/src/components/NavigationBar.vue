<template>
  <div id="navigation-bar">
    <div class="search-bar">
      <input type="search" placeholder="Search for a key…" v-model="needle" />
      <IconSearch />
    </div>
    <ul class="nav-translation-key-list">
      <li
        v-if="needle.length == 0 && namespace.length > 0"
        class="nav-translation-key nav-back"
      >
        <a href="#" @click.prevent="goToParentNamespace(namespace)">
          <IconChevronRight />
          <span
            ><b>{{ namespace }}</b></span
          >
        </a>
      </li>
      <template v-for="key in filteredTranslationsKeys" :key="key">
        <NavigationKey
          :canonicalKey="key"
          :namespace="namespace"
          :forceCanonical="needle.length > 0"
          @namespaceChanged="setNamespace(...arguments)"
        />
      </template>
    </ul>
  </div>
</template>

<script>
import { IconChevronRight, IconSearch } from "../assets/Icons.jsx";

import NavigationKey from "./NavigationKey.vue";
import _ from "lodash";

export default {
  name: "navigation-bar",

  data() {
    return {
      needle: "",
    };
  },

  props: {
    translationKeys: Array,
    namespace: String,
  },

  computed: {
    filteredTranslationsKeys() {
      if (this.$data.needle !== "") {
        return this.foundTranslationKeys;
      } else {
        return this.currentNamespaceTranslationKeys;
      }
    },

    currentNamespaceTranslationKeys() {
      let currentKeys = _.filter(this.translationKeys, (key) => {
        return key.startsWith(this.namespace || "");
      });

      let truncated = _.map(currentKeys, (key) => {
        let namespaceDepth = 0;
        if (this.namespace.length > 0) {
          namespaceDepth = (this.namespace.match(/\./g) || []).length + 1;
        }
        let keyWithoutNamespace = key
          .split(".")
          .slice(namespaceDepth)
          .join(".");
        let crumbs = keyWithoutNamespace.split(".");
        let initialDot = this.namespace.length > 0 ? "." : "";

        if (crumbs.length > 1) {
          return this.namespace + initialDot + crumbs[0] + "..";
        } else {
          return this.namespace + initialDot + keyWithoutNamespace;
        }
      });

      return _.uniq(truncated).sort();
    },

    foundTranslationKeys() {
      return _.filter(this.translationKeys, (key) => {
        return key.includes(this.$data.needle);
      }).sort();
    },
  },

  methods: {
    setNamespace(namespace) {
      this.$emit("namespaceChanged", namespace);
    },
    goToParentNamespace(namespace) {
      this.$emit(
        "namespaceChanged",
        namespace.split(".").slice(0, -1).join(".")
      );
    },
  },

  components: {
    NavigationKey,
    IconChevronRight,
    IconSearch,
  },
};
</script>

<style>
#navigation-bar {
  display: flex;
  flex-direction: column;
  width: 260px;
  background-color: #e6eaef;
  border-right: 1px solid #777777;
}

.search-bar {
  flex: 0 0 39px;
  display: flex;
  background-color: lightgrey;
  border-bottom: 1px solid grey;
  position: relative;
}

.search-bar input + svg {
  font-size: 9pt;
  position: absolute;
  left: 10px;
  top: 50%;
  transform: translateY(-50%) !important;
  color: grey;
  width: 16px;
  height: 16px;
}

.search-bar input {
  flex: 1;
  margin: 5px;
  border-radius: 3px;
  border: 1px solid grey;
  padding-left: 24px;
}

.search-bar input:focus + svg {
  color: black;
}
</style>
