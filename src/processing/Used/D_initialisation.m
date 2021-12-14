addpath(".");

close all
clear
clc
format long g

root_path = "/media/joebillingsley/Data/projects/NFV_PlacementModel_Journal";
% root_path = "D:\Research\NFV_PlacementModel_Journal";

src_folder = fullfile(root_path, "processed", "aggregate", "Initialisation");
out_folder = fullfile(root_path, "processed", "initialisation");

if ~exist(out_folder, 'dir')
    mkdir(out_folder)
end

sizes = ["12", "16", "20", "24", "28", "32"];
ops = ["std","ca", "uni"];

for i = 1:length(ops)
    rows = [];
    for j = 1:length(sizes)
        hv_file = fullfile(src_folder, sizes(j), ops(i), 'aggregate.csv');
        agg = csvread(hv_file);
        
        median = agg(end, 6);
        
        lq = median - agg(end, 5);
        uq = agg(end, 7) - median;
        
        k = str2double(sizes(j));
        rows(j, :) = [(k ^ 3) / 4; lq; median; uq];
    end
    
    out_file = fullfile(out_folder, append(ops(i), '.csv'));
    fid = fopen(out_file, 'w');
    
    csvwrite(out_file, rows);
end