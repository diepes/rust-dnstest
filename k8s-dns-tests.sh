#!/usr/bin/env bash

action="$1"  # First argument passed to the script
# Array containing pod names and parameters
source k8s-pod-info.pvt.sh
if [[ "$pods_ns" == "" ]]; then
    echo "Err no \$pod_ns"
    exit 1
fi
counter=0
# Loop through the array and start pods
for pod in "${pods[@]}"; do
    ((counter++))
    # Split the pod entry into pod name and parameters
    IFS=" " read -r -a pod_info <<< "$pod"
    pod_name="${pod_info[0]}"
    pod_params="${pod_info[@]:1}"
    k8s_filter=""

    # Create kubectl command
    if [[ "$action" == "run" ]]; then
        k8s_cmd="kubectl run -n $pods_ns \"$pod_name\" $pod_params"

    elif [[ "$action" == "delete" ]]; then
        k8s_cmd="kubectl delete -n $pods_ns pod \"$pod_name\""

    elif [[ "$action" == "logs" ]]; then
        f=$(mktemp /tmp/dns-test-script.XXXXXX)
        k8s_cmd="kubectl logs -n $pods_ns \"$pod_name\""
        k8s_filter="> ${f}; grep \"Time\" ${f} ; grep \"^msec:\" ${f} |tail -n1;"
        rm ${f}
    else
        echo "Invalid action. Use 'run', 'logs' or 'delete'."
        exit 1

    fi
    # Run kubectl command
    echo "##  run${counter}: $k8s_cmd"
    output=$(eval "$k8s_cmd $k8s_filter")
    echo "$output"
    echo
done
echo
